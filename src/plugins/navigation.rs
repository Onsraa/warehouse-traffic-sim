use bevy::prelude::*;

use crate::constants::TICK_DELTA;
use crate::core::{HighwayGraph, SpaceTimeTable};
use crate::systems::navigation::{
    battery_consumption_system, deadlock_detection_system, path_execution_system,
    simulation_tick_system, visual_interpolation_system,
};
use crate::systems::pbs::{pbs_planning_system, update_priorities_system, PbsConfig};
use crate::systems::spawner::{mission_progression_system, sequential_spawn_system, SpawnQueue};

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpaceTimeTable>()
            .init_resource::<HighwayGraph>()
            .init_resource::<PbsConfig>()
            .init_resource::<SpawnQueue>()
            .add_systems(
                FixedUpdate,
                (
                    simulation_tick_system,
                    sequential_spawn_system,
                    mission_progression_system,
                    update_priorities_system,
                    pbs_planning_system,
                    path_execution_system,
                    deadlock_detection_system,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (visual_interpolation_system, battery_consumption_system),
            )
            .insert_resource(Time::<Fixed>::from_seconds(TICK_DELTA as f64));
    }
}