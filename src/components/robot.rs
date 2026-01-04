use bevy::prelude::*;
use crate::core::GridPos;

/// État opérationnel du robot
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RobotState {
    #[default]
    Idle,
    Moving,
    Loading,
    Unloading,
    Charging,
    Fault,
}

impl RobotState {
    /// Priorité de base selon l'état (plus haut = plus prioritaire)
    pub fn base_priority(&self) -> u8 {
        match self {
            Self::Fault => 0,      
            Self::Loading | Self::Unloading => 10,
            Self::Moving => 20,
            Self::Idle => 30,
            Self::Charging => 40,
        }
    }
}

/// Marqueur principal robot
#[derive(Component)]
#[require(GridPosition, State, Priority, Loaded, Battery, PlannedPath, Velocity)]
pub struct Robot;

/// Position actuelle sur la grille
#[derive(Component, Default, Clone, Copy)]
pub struct GridPosition(pub GridPos);

/// Destination finale
#[derive(Component)]
pub struct Destination(pub GridPos);

/// État du robot
#[derive(Component, Default)]
pub struct State(pub RobotState);

/// Priorité dynamique (0 = max priorité)
#[derive(Component, Default)]
pub struct Priority(pub u8);

/// Indique si le robot transporte une charge
#[derive(Component, Default)]
pub struct Loaded(pub bool);

/// Niveau de batterie (0.0 - 1.0)
#[derive(Component)]
pub struct Battery(pub f32);

impl Default for Battery {
    fn default() -> Self {
        Self(1.0)
    }
}

/// Trajectoire planifiée (positions + ticks)
#[derive(Component, Default)]
pub struct PlannedPath {
    pub waypoints: Vec<(GridPos, u64)>,
    pub current_index: usize,
}

impl PlannedPath {
    pub fn new(waypoints: Vec<(GridPos, u64)>) -> Self {
        Self { waypoints, current_index: 0 }
    }

    pub fn clear(&mut self) {
        self.waypoints.clear();
        self.current_index = 0;
    }

    pub fn current(&self) -> Option<(GridPos, u64)> {
        self.waypoints.get(self.current_index).copied()
    }

    pub fn advance(&mut self) {
        if self.current_index < self.waypoints.len() {
            self.current_index += 1;
        }
    }

    pub fn is_complete(&self) -> bool {
        self.current_index >= self.waypoints.len()
    }

    pub fn remaining(&self) -> &[(GridPos, u64)] {
        &self.waypoints[self.current_index..]
    }
}

/// Vélocité actuelle (pour interpolation visuelle)
#[derive(Component, Default)]
pub struct Velocity(pub f32);