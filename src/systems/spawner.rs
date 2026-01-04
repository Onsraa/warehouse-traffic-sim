use bevy::prelude::*;

use crate::components::{Destination, GridPosition, Loaded, Mission, MissionPhase, Robot, RobotState, State};
use crate::constants::ROBOT_COUNT;
use crate::core::{SpaceTimeTable, WarehouseGrid, WarehouseZones};

#[derive(Resource)]
pub struct SpawnQueue {
    pub total: u32,
    pub spawned_count: u32,
    pub cooldown_ticks: u64,
    pub last_spawn_tick: u64,
}

impl Default for SpawnQueue {
    fn default() -> Self {
        Self {
            total: ROBOT_COUNT,
            spawned_count: 0,
            cooldown_ticks: 15,
            last_spawn_tick: 0,
        }
    }
}

impl SpawnQueue {
    pub fn is_complete(&self) -> bool {
        self.spawned_count >= self.total
    }
}

pub fn sequential_spawn_system(
    mut commands: Commands,
    mut queue: ResMut<SpawnQueue>,
    mut zones: ResMut<WarehouseZones>,
    space_time: Res<SpaceTimeTable>,
    grid: Res<WarehouseGrid>,
    robots: Query<&GridPosition, With<Robot>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if queue.is_complete() {
        return;
    }

    let current_tick = space_time.current_tick();

    if current_tick < queue.last_spawn_tick + queue.cooldown_ticks {
        return;
    }

    let spawn_pos = zones.next_spawn();

    if robots.iter().any(|pos| pos.0 == spawn_pos) {
        return;
    }

    let storage_target = zones.next_storage();
    let cargo_target = zones.next_cargo();

    let (wx, wz) = grid.grid_to_world(spawn_pos);

    let mesh = meshes.add(Cuboid::new(0.6, 0.4, 0.6));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.6, 0.2), // Vert = non chargé
        ..default()
    });

    commands.spawn((
        Robot,
        GridPosition(spawn_pos),
        Destination(storage_target),
        State(RobotState::Moving),
        Loaded(false),
        Mission::new(storage_target, cargo_target),
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_xyz(wx, 0.2, wz),
    ));

    queue.spawned_count += 1;
    queue.last_spawn_tick = current_tick;
}

pub fn mission_progression_system(
    mut robots: Query<(
        &GridPosition,
        &mut Mission,
        &mut Destination,
        &mut State,
        &mut Loaded,
    ), With<Robot>>,
    mut zones: ResMut<WarehouseZones>,
) {
    for (pos, mut mission, mut dest, mut state, mut loaded) in &mut robots {
        match mission.phase {
            MissionPhase::GoingToStorage => {
                if pos.0 == mission.storage_target {
                    // Arrivé au storage → charge l'objet → va au cargo
                    loaded.0 = true;
                    mission.phase = MissionPhase::GoingToCargo;
                    dest.0 = mission.cargo_target;
                    state.0 = RobotState::Moving;
                }
            }
            MissionPhase::GoingToCargo => {
                if pos.0 == mission.cargo_target {
                    // Arrivé au cargo → décharge → nouvelle mission
                    loaded.0 = false;

                    // Assigne nouvelle mission
                    let new_storage = zones.next_storage();
                    let new_cargo = zones.next_cargo();

                    mission.storage_target = new_storage;
                    mission.cargo_target = new_cargo;
                    mission.phase = MissionPhase::GoingToStorage;
                    dest.0 = new_storage;
                    state.0 = RobotState::Moving;
                }
            }
        }
    }
}