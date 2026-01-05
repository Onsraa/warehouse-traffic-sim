use bevy::prelude::*;

use crate::components::{
    Battery, GridPosition, Mission, MissionPhase, PlannedPath, Robot, RobotState, State, Velocity,
};
use crate::constants::{ROBOT_ACCELERATION, ROBOT_DECELERATION, ROBOT_MAX_VELOCITY};
use crate::core::{SpaceTimeTable, WarehouseGrid};

pub fn path_execution_system(
    mut robots: Query<(&mut GridPosition, &mut PlannedPath, &Mission, &mut Velocity), With<Robot>>,
    space_time: Res<SpaceTimeTable>,
) {
    let current_tick = space_time.current_tick();

    for (mut grid_pos, mut path, mission, mut vel) in &mut robots {
        if path.is_complete() {
            vel.0 = 0.0;
            continue;
        }

        if let Some((next_pos, target_tick)) = path.current() {
            if current_tick >= target_tick {
                grid_pos.0 = next_pos;
                path.advance();
            }
        }
    }
}

pub fn visual_interpolation_system(
    mut robots: Query<(&GridPosition, &PlannedPath, &mut Transform, &mut Velocity), With<Robot>>,
    grid: Res<WarehouseGrid>,
    space_time: Res<SpaceTimeTable>,
    time: Res<Time>,
) {
    let current_tick = space_time.current_tick();

    for (grid_pos, path, mut transform, mut vel) in &mut robots {
        let current_world = grid.grid_to_world(grid_pos.0);
        let target = Vec3::new(current_world.0, 0.2, current_world.1);

        if let Some((next_pos, target_tick)) = path.current() {
            if target_tick > current_tick {
                let next_world = grid.grid_to_world(next_pos);
                let next_target = Vec3::new(next_world.0, 0.2, next_world.1);

                let t = time.delta_secs() * 10.0;
                transform.translation = transform.translation.lerp(next_target, t);

                vel.0 = (vel.0 + ROBOT_ACCELERATION * time.delta_secs()).min(ROBOT_MAX_VELOCITY);
            } else {
                transform.translation = target;
            }
        } else {
            vel.0 = (vel.0 - ROBOT_DECELERATION * time.delta_secs()).max(0.0);
            transform.translation = transform.translation.lerp(target, 5.0 * time.delta_secs());
        }
    }
}

pub fn battery_consumption_system(
    mut robots: Query<(&State, &Velocity, &mut Battery), With<Robot>>,
    time: Res<Time>,
) {
    for (state, vel, mut battery) in &mut robots {
        let consumption = match state.0 {
            RobotState::Moving => 0.0001 * vel.0 * vel.0 * time.delta_secs(),
            RobotState::Idle => 0.00001 * time.delta_secs(),
            RobotState::Charging => -0.001 * time.delta_secs(),
            _ => 0.00005 * time.delta_secs(),
        };

        battery.0 = (battery.0 - consumption).clamp(0.0, 1.0);
    }
}

pub fn simulation_tick_system(mut space_time: ResMut<SpaceTimeTable>) {
    space_time.advance_tick();
}

pub fn deadlock_detection_system(
    robots: Query<(Entity, &GridPosition, &PlannedPath, &State), With<Robot>>,
    mut space_time: ResMut<SpaceTimeTable>,
) {
    for (entity, pos, path, state) in &robots {
        if matches!(state.0, RobotState::Moving) && path.remaining().is_empty() {
            warn!("Potential deadlock: {:?} at {:?}", entity, pos.0);
            space_time.clear_entity(entity);
        }
    }
}
