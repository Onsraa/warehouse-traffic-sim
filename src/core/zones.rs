use bevy::prelude::*;
use super::GridPos;
use crate::constants::{GRID_WIDTH, GRID_HEIGHT, SPAWN_ZONE_WIDTH, CARGO_ZONE_WIDTH, STORAGE_MARGIN};

#[derive(Resource)]
pub struct WarehouseZones {
    pub spawn_points: Vec<GridPos>,
    pub storage_cells: Vec<GridPos>,
    pub cargo_cells: Vec<GridPos>,
    spawn_index: usize,
    storage_index: usize,
    cargo_index: usize,
}

impl Default for WarehouseZones {
    fn default() -> Self {
        let mut spawn_points = Vec::new();
        let mut storage_cells = Vec::new();
        let mut cargo_cells = Vec::new();

        // Spawns: zone gauche
        for x in 1..SPAWN_ZONE_WIDTH as i32 - 1 {
            for y in (1..GRID_HEIGHT as i32 - 1).step_by(3) {
                spawn_points.push(GridPos::new(x, y));
            }
        }

        // Storage: zone centrale
        let storage_start_x = STORAGE_MARGIN as i32;
        let storage_end_x = (GRID_WIDTH - CARGO_ZONE_WIDTH - 5) as i32;
        let storage_start_y = 5i32;
        let storage_end_y = (GRID_HEIGHT - 5) as i32;

        for x in (storage_start_x..storage_end_x).step_by(4) {
            for y in (storage_start_y..storage_end_y).step_by(4) {
                storage_cells.push(GridPos::new(x, y));
            }
        }

        // Cargo: zone droite
        let cargo_start_x = (GRID_WIDTH - CARGO_ZONE_WIDTH + 1) as i32;
        for x in cargo_start_x..(GRID_WIDTH as i32 - 1) {
            for y in (1..GRID_HEIGHT as i32 - 1).step_by(3) {
                cargo_cells.push(GridPos::new(x, y));
            }
        }

        Self {
            spawn_points,
            storage_cells,
            cargo_cells,
            spawn_index: 0,
            storage_index: 0,
            cargo_index: 0,
        }
    }
}

impl WarehouseZones {
    pub fn next_spawn(&mut self) -> GridPos {
        let pos = self.spawn_points[self.spawn_index % self.spawn_points.len()];
        self.spawn_index += 1;
        pos
    }

    pub fn next_storage(&mut self) -> GridPos {
        let pos = self.storage_cells[self.storage_index % self.storage_cells.len()];
        self.storage_index += 1;
        pos
    }

    pub fn next_cargo(&mut self) -> GridPos {
        let pos = self.cargo_cells[self.cargo_index % self.cargo_cells.len()];
        self.cargo_index += 1;
        pos
    }
}