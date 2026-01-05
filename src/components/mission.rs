use bevy::prelude::*;
use crate::core::GridPos;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MissionPhase {
    #[default]
    GoingToStorage,
    PickingUp,
    GoingToCargo,
    DroppingOff,
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

/// Timer pour les actions de chargement/déchargement
#[derive(Component)]
pub struct ActionTimer {
    pub remaining: f32,
}

impl ActionTimer {
    pub fn new(duration: f32) -> Self {
        Self { remaining: duration }
    }

    pub fn tick(&mut self, delta: f32) -> bool {
        self.remaining -= delta;
        self.remaining <= 0.0
    }

    pub fn progress(&self, total: f32) -> f32 {
        1.0 - (self.remaining / total).clamp(0.0, 1.0)
    }
}