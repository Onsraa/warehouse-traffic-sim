use bevy::prelude::*;

use crate::components::{
    ActionTimer, Destination, GridPosition, Loaded, Mission, MissionPhase,
    Robot, RobotState, State,
};
use crate::constants::{DROPOFF_DURATION, PICKUP_DURATION, ROBOT_COUNT};
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
            cooldown_ticks: 20,
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

    // Réserve storage et cargo - skip si aucun disponible
    let Some(storage_target) = zones.reserve_storage() else {
        return;
    };
    let Some(cargo_target) = zones.reserve_cargo() else {
        zones.release_storage(storage_target);
        return;
    };

    let (wx, wz) = grid.grid_to_world(spawn_pos);

    let mesh = meshes.add(Cuboid::new(0.6, 0.4, 0.6));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.6, 0.2),
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
    mut commands: Commands,
    mut robots: Query<(
        Entity,
        &GridPosition,
        &mut Mission,
        &mut Destination,
        &mut State,
        &mut Loaded,
        Option<&mut ActionTimer>,
    ), With<Robot>>,
    mut zones: ResMut<WarehouseZones>,
    time: Res<Time>,
) {
    for (entity, pos, mut mission, mut dest, mut state, mut loaded, timer) in &mut robots {
        match mission.phase {
            MissionPhase::GoingToStorage => {
                if pos.0 == mission.storage_target {
                    mission.phase = MissionPhase::PickingUp;
                    state.0 = RobotState::Loading;
                    commands.entity(entity).insert(ActionTimer::new(PICKUP_DURATION));
                }
            }
            MissionPhase::PickingUp => {
                if let Some(mut t) = timer {
                    if t.tick(time.delta_secs()) {
                        loaded.0 = true;
                        mission.phase = MissionPhase::GoingToCargo;
                        dest.0 = mission.cargo_target;
                        state.0 = RobotState::Moving;
                        commands.entity(entity).remove::<ActionTimer>();

                        // Libère le storage
                        zones.release_storage(mission.storage_target);
                    }
                }
            }
            MissionPhase::GoingToCargo => {
                if pos.0 == mission.cargo_target {
                    mission.phase = MissionPhase::DroppingOff;
                    state.0 = RobotState::Unloading;
                    commands.entity(entity).insert(ActionTimer::new(DROPOFF_DURATION));
                }
            }
            MissionPhase::DroppingOff => {
                if let Some(mut t) = timer {
                    if t.tick(time.delta_secs()) {
                        loaded.0 = false;
                        commands.entity(entity).remove::<ActionTimer>();

                        // Libère le cargo actuel
                        zones.release_cargo(mission.cargo_target);

                        // Réserve nouvelle mission
                        let new_storage = zones.reserve_storage();
                        let new_cargo = zones.reserve_cargo();

                        match (new_storage, new_cargo) {
                            (Some(storage), Some(cargo)) => {
                                mission.storage_target = storage;
                                mission.cargo_target = cargo;
                                mission.phase = MissionPhase::GoingToStorage;
                                dest.0 = storage;
                                state.0 = RobotState::Moving;
                            }
                            (Some(storage), None) => {
                                // Pas de cargo dispo, libère storage et attend
                                zones.release_storage(storage);
                                state.0 = RobotState::Idle;
                            }
                            (None, Some(cargo)) => {
                                // Pas de storage dispo, libère cargo et attend
                                zones.release_cargo(cargo);
                                state.0 = RobotState::Idle;
                            }
                            (None, None) => {
                                // Rien de dispo, attend
                                state.0 = RobotState::Idle;
                            }
                        }
                    }
                }
            }
        }
    }
}