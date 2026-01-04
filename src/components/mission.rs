use bevy::prelude::*;
use crate::core::GridPos;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MissionPhase {
    #[default]
    GoingToStorage,
    GoingToCargo,
}

#[derive(Component)]
pub struct Mission {
    pub phase: MissionPhase,
    pub storage_target: GridPos,
    pub cargo_target: GridPos,
}

impl Mission {
    pub fn new(storage: GridPos, cargo: GridPos) -> Self {
        Self {
            phase: MissionPhase::GoingToStorage,
            storage_target: storage,
            cargo_target: cargo,
        }
    }
}