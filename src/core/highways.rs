use bevy::prelude::*;
use super::{Direction, GridPos};
use crate::constants::{GRID_HEIGHT, GRID_WIDTH, SPAWN_ZONE_WIDTH};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZoneType {
    FreeZone,
    Highway,
    Storage,
}

#[derive(Resource)]
pub struct HighwayGraph {
    width: u32,
    height: u32,
}

impl Default for HighwayGraph {
    fn default() -> Self {
        Self::new(GRID_WIDTH, GRID_HEIGHT)
    }
}

impl HighwayGraph {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    #[inline]
    pub fn zone_type(&self, pos: GridPos) -> ZoneType {
        let x = pos.x as u32;

        if x < SPAWN_ZONE_WIDTH {
            return ZoneType::FreeZone;
        }

        ZoneType::Highway
    }

    /// Toutes directions autorisées pour l'instant (on ajoutera les contraintes plus tard)
    #[inline]
    pub fn allowed_directions(&self, _pos: GridPos) -> &'static [Direction] {
        &[Direction::North, Direction::South, Direction::East, Direction::West]
    }

    #[inline]
    pub fn is_move_legal(&self, from: GridPos, to: GridPos) -> bool {
        let dx = to.x - from.x;
        let dy = to.y - from.y;
        dx.abs() + dy.abs() == 1 && self.in_bounds(to)
    }

    pub fn legal_neighbors(&self, pos: GridPos) -> impl Iterator<Item = GridPos> + '_ {
        Direction::CARDINALS
            .iter()
            .map(move |&dir| pos.neighbor(dir))
            .filter(|&n| self.in_bounds(n))
    }

    #[inline]
    pub fn in_bounds(&self, pos: GridPos) -> bool {
        pos.x >= 0 && pos.y >= 0 && (pos.x as u32) < self.width && (pos.y as u32) < self.height
    }
}