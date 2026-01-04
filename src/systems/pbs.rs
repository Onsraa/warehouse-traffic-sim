use bevy::prelude::*;
use rustc_hash::FxHashMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::components::{
    Destination, GridPosition, Loaded, PlannedPath, Priority, Robot, RobotState, State,
};
use crate::core::{GridPos, HighwayGraph, SpaceTimeTable, WarehouseGrid};

#[derive(Resource)]
pub struct PbsConfig {
    pub horizon: u64,
    pub replan_interval: u64,
    pub heuristic_weight: f32,
}

impl Default for PbsConfig {
    fn default() -> Self {
        Self {
            horizon: 100,
            replan_interval: 3,
            heuristic_weight: 1.2,
        }
    }
}

#[derive(Clone)]
struct SpaceTimeNode {
    pos: GridPos,
    tick: u64,
    g_cost: f32,
    f_cost: f32,
    parent: Option<(GridPos, u64)>,
}

impl PartialEq for SpaceTimeNode {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.tick == other.tick
    }
}

impl Eq for SpaceTimeNode {}

impl Ord for SpaceTimeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .f_cost
            .partial_cmp(&self.f_cost)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for SpaceTimeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Set des positions occupées par des robots statiques
#[derive(Default)]
pub struct StaticObstacles {
    positions: FxHashMap<GridPos, Entity>,
}

impl StaticObstacles {
    pub fn is_blocked(&self, pos: GridPos, exclude: Option<Entity>) -> bool {
        match self.positions.get(&pos) {
            None => false,
            Some(&e) => exclude.map_or(true, |ex| ex != e),
        }
    }
}

pub struct PbsPlanner<'a> {
    grid: &'a WarehouseGrid,
    highways: &'a HighwayGraph,
    space_time: &'a SpaceTimeTable,
    static_obstacles: &'a StaticObstacles,
    config: &'a PbsConfig,
}

impl<'a> PbsPlanner<'a> {
    pub fn new(
        grid: &'a WarehouseGrid,
        highways: &'a HighwayGraph,
        space_time: &'a SpaceTimeTable,
        static_obstacles: &'a StaticObstacles,
        config: &'a PbsConfig,
    ) -> Self {
        Self {
            grid,
            highways,
            space_time,
            static_obstacles,
            config,
        }
    }

    pub fn plan_path(
        &self,
        start: GridPos,
        goal: GridPos,
        start_tick: u64,
        entity: Entity,
    ) -> Option<Vec<(GridPos, u64)>> {
        if start == goal {
            return Some(vec![(start, start_tick)]);
        }

        let horizon_end = start_tick + self.config.horizon;
        let mut open = BinaryHeap::new();
        let mut closed: FxHashMap<(GridPos, u64), SpaceTimeNode> = FxHashMap::default();

        let h = self.heuristic(start, goal);
        open.push(SpaceTimeNode {
            pos: start,
            tick: start_tick,
            g_cost: 0.0,
            f_cost: h,
            parent: None,
        });

        let mut iterations = 0;
        let max_iterations = 10000;
        let mut best_node: Option<SpaceTimeNode> = None;

        while let Some(current) = open.pop() {
            iterations += 1;
            if iterations > max_iterations {
                warn!("PBS: max iterations reached for {:?}", entity);
                break;
            }

            if current.pos == goal {
                return Some(self.reconstruct_path(&closed, current));
            }

            // Garde le meilleur nœud trouvé
            if best_node.as_ref().map_or(true, |b| {
                current.pos.manhattan_distance(&goal) < b.pos.manhattan_distance(&goal)
            }) {
                best_node = Some(current.clone());
            }

            if current.tick >= horizon_end {
                continue;
            }

            let key = (current.pos, current.tick);
            if closed.contains_key(&key) {
                continue;
            }
            closed.insert(key, current.clone());

            let next_tick = current.tick + 1;

            // Attendre
            if self.is_valid_wait(&current.pos, next_tick, entity) {
                self.try_add_neighbor(
                    &mut open,
                    &closed,
                    current.pos,
                    next_tick,
                    current.g_cost + 0.5,
                    goal,
                    current.pos,
                    current.tick,
                );
            }

            // Se déplacer
            for neighbor in self.highways.legal_neighbors(current.pos) {
                if !self.grid.is_passable(neighbor) {
                    continue;
                }
                if self.static_obstacles.is_blocked(neighbor, Some(entity)) {
                    continue;
                }
                if !self.is_valid_move(&current.pos, &neighbor, current.tick, next_tick, entity) {
                    continue;
                }

                self.try_add_neighbor(
                    &mut open,
                    &closed,
                    neighbor,
                    next_tick,
                    current.g_cost + 1.0,
                    goal,
                    current.pos,
                    current.tick,
                );
            }
        }

        // Retourne le meilleur chemin partiel trouvé
        if let Some(node) = best_node {
            let path = self.reconstruct_path(&closed, node);
            if path.len() > 1 {
                return Some(path);
            }
        }

        warn!(
            "PBS: no path found for {:?} from {:?} to {:?}",
            entity, start, goal
        );
        None
    }

    fn is_valid_wait(&self, pos: &GridPos, to_tick: u64, entity: Entity) -> bool {
        self.space_time.is_free(*pos, to_tick, Some(entity))
    }

    fn is_valid_move(
        &self,
        from: &GridPos,
        to: &GridPos,
        from_tick: u64,
        to_tick: u64,
        entity: Entity,
    ) -> bool {
        if !self.space_time.is_free(*to, to_tick, Some(entity)) {
            return false;
        }
        self.space_time
            .is_edge_free(*from, *to, from_tick, Some(entity))
    }

    fn try_add_neighbor(
        &self,
        open: &mut BinaryHeap<SpaceTimeNode>,
        closed: &FxHashMap<(GridPos, u64), SpaceTimeNode>,
        pos: GridPos,
        tick: u64,
        g_cost: f32,
        goal: GridPos,
        parent_pos: GridPos,
        parent_tick: u64,
    ) {
        if closed.contains_key(&(pos, tick)) {
            return;
        }

        let h = self.heuristic(pos, goal) * self.config.heuristic_weight;
        open.push(SpaceTimeNode {
            pos,
            tick,
            g_cost,
            f_cost: g_cost + h,
            parent: Some((parent_pos, parent_tick)),
        });
    }

    #[inline]
    fn heuristic(&self, from: GridPos, to: GridPos) -> f32 {
        from.manhattan_distance(&to) as f32
    }

    fn reconstruct_path(
        &self,
        closed: &FxHashMap<(GridPos, u64), SpaceTimeNode>,
        end: SpaceTimeNode,
    ) -> Vec<(GridPos, u64)> {
        let mut path = vec![(end.pos, end.tick)];
        let mut current = end.parent;

        while let Some((pos, tick)) = current {
            path.push((pos, tick));
            current = closed.get(&(pos, tick)).and_then(|n| n.parent);
        }

        path.reverse();
        path
    }
}

/// Système de planification PBS
pub fn pbs_planning_system(
    mut robots: Query<
        (
            Entity,
            &GridPosition,
            &Destination,
            &Priority,
            &Loaded,
            &State,
            &mut PlannedPath,
        ),
        With<Robot>,
    >,
    grid: Res<WarehouseGrid>,
    highways: Res<HighwayGraph>,
    mut space_time: ResMut<SpaceTimeTable>,
    config: Res<PbsConfig>,
) {
    let current_tick = space_time.current_tick();

    if current_tick % config.replan_interval != 0 {
        return;
    }

    // Construit la liste des obstacles statiques (robots Idle ou AtStation)
    let mut static_obstacles = StaticObstacles::default();
    for (entity, grid_pos, _, _, _, state, _) in &robots {
        if matches!(state.0, RobotState::Idle) {
            static_obstacles.positions.insert(grid_pos.0, entity);
        }
    }

    // Trie par priorité
    let mut sorted_robots: Vec<_> = robots.iter_mut().collect();
    sorted_robots.sort_by_key(|(_, _, _, prio, loaded, _, _)| {
        let load_bonus = if loaded.0 { 0u8 } else { 50 };
        prio.0.saturating_add(load_bonus)
    });

    space_time.cleanup(current_tick);

    // Réserve d'abord les positions des robots statiques pour tous les ticks de l'horizon
    for (entity, pos, _, _, _, state, _) in &sorted_robots {
        if matches!(state.0, RobotState::Idle) {
            for tick in current_tick..current_tick + config.horizon {
                space_time.reserve(pos.0, tick, *entity);
            }
        }
    }

    // Planifie les robots mobiles
    for (entity, grid_pos, dest, _, _, state, mut path) in sorted_robots {
        // Skip les robots statiques
        if matches!(state.0, RobotState::Idle) {
            continue;
        }

        space_time.clear_entity(entity);

        let planner = PbsPlanner::new(&grid, &highways, &space_time, &static_obstacles, &config);

        if let Some(new_path) = planner.plan_path(grid_pos.0, dest.0, current_tick, entity) {
            space_time.reserve_path(&new_path, entity);
            *path = PlannedPath::new(new_path);
        }
        // Si None: le robot reste figé avec son chemin actuel (ou vide)
    }
}

pub fn update_priorities_system(
    mut robots: Query<(&State, &Loaded, &crate::components::Battery, &mut Priority), With<Robot>>,
) {
    for (state, loaded, battery, mut priority) in &mut robots {
        let mut p = state.0.base_priority();

        if loaded.0 {
            p = p.saturating_sub(15);
        }

        if battery.0 < 0.2 {
            p = p.saturating_sub(10);
        }

        priority.0 = p;
    }
}
