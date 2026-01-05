use bevy::prelude::*;
use rustc_hash::FxHashSet;
use super::GridPos;
use crate::constants::{
    GRID_WIDTH, GRID_HEIGHT, SPAWN_ZONE_WIDTH, CARGO_ZONE_WIDTH,
    RACK_LENGTH, AISLE_WIDTH,
};

#[derive(Clone, Copy)]
pub struct Rack {
    pub start: GridPos,
    pub end: GridPos,
}

impl Rack {
    pub fn contains(&self, pos: GridPos) -> bool {
        pos.x >= self.start.x && pos.x <= self.end.x &&
            pos.y >= self.start.y && pos.y <= self.end.y
    }
}

#[derive(Resource)]
pub struct WarehouseZones {
    pub spawn_points: Vec<GridPos>,
    pub storage_cells: Vec<GridPos>,
    pub cargo_cells: Vec<GridPos>,
    pub racks: Vec<Rack>,

    // Réservations actives
    reserved_storage: FxHashSet<GridPos>,
    reserved_cargo: FxHashSet<GridPos>,

    spawn_index: usize,
    storage_index: usize,
    cargo_index: usize,
}

impl Default for WarehouseZones {
    fn default() -> Self {
        let mut spawn_points = Vec::new();
        let mut storage_cells = Vec::new();
        let mut cargo_cells = Vec::new();
        let mut racks = Vec::new();

        // Zone de spawn (gauche)
        for x in 1..SPAWN_ZONE_WIDTH as i32 - 1 {
            for y in (2..GRID_HEIGHT as i32 - 2).step_by(4) {
                spawn_points.push(GridPos::new(x, y));
            }
        }

        // Zone de cargo (droite)
        let cargo_start_x = (GRID_WIDTH - CARGO_ZONE_WIDTH + 1) as i32;
        for x in cargo_start_x..(GRID_WIDTH as i32 - 1) {
            for y in (2..GRID_HEIGHT as i32 - 2).step_by(3) {
                cargo_cells.push(GridPos::new(x, y));
            }
        }

        // Zone de stockage avec longs couloirs
        let storage_start_x = SPAWN_ZONE_WIDTH as i32 + 3;
        let storage_end_x = (GRID_WIDTH - CARGO_ZONE_WIDTH - 3) as i32;
        let storage_start_y = 3i32;
        let storage_end_y = (GRID_HEIGHT - 3) as i32;

        let row_spacing = RACK_LENGTH as i32 + AISLE_WIDTH as i32 + 1;
        let mut current_y = storage_start_y;

        while current_y + RACK_LENGTH as i32 <= storage_end_y {
            let rack_y_start = current_y;
            let rack_y_end = current_y + RACK_LENGTH as i32 - 1;

            let mut rack_x = storage_start_x;
            let rack_width = 2;
            let pair_spacing = 3;

            while rack_x + rack_width <= storage_end_x {
                let rack = Rack {
                    start: GridPos::new(rack_x, rack_y_start),
                    end: GridPos::new(rack_x + rack_width - 1, rack_y_end),
                };
                racks.push(rack);

                for y in rack_y_start..=rack_y_end {
                    let left_access = GridPos::new(rack_x - 1, y);
                    if left_access.x >= storage_start_x - 1 {
                        storage_cells.push(left_access);
                    }
                    let right_access = GridPos::new(rack_x + rack_width, y);
                    if right_access.x <= storage_end_x + 1 {
                        storage_cells.push(right_access);
                    }
                }

                rack_x += rack_width + pair_spacing;
            }

            current_y += row_spacing;
        }

        storage_cells.sort_by(|a, b| (a.x, a.y).cmp(&(b.x, b.y)));
        storage_cells.dedup();
        storage_cells.retain(|pos| !racks.iter().any(|r| r.contains(*pos)));

        Self {
            spawn_points,
            storage_cells,
            cargo_cells,
            racks,
            reserved_storage: FxHashSet::default(),
            reserved_cargo: FxHashSet::default(),
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

    /// Réserve un storage libre, retourne None si tous occupés
    pub fn reserve_storage(&mut self) -> Option<GridPos> {
        let len = self.storage_cells.len();
        for i in 0..len {
            let idx = (self.storage_index + i) % len;
            let pos = self.storage_cells[idx];
            if !self.reserved_storage.contains(&pos) {
                self.reserved_storage.insert(pos);
                self.storage_index = idx + 1;
                return Some(pos);
            }
        }
        None
    }

    /// Réserve un cargo libre, retourne None si tous occupés
    pub fn reserve_cargo(&mut self) -> Option<GridPos> {
        let len = self.cargo_cells.len();
        for i in 0..len {
            let idx = (self.cargo_index + i) % len;
            let pos = self.cargo_cells[idx];
            if !self.reserved_cargo.contains(&pos) {
                self.reserved_cargo.insert(pos);
                self.cargo_index = idx + 1;
                return Some(pos);
            }
        }
        None
    }

    /// Libère un storage
    pub fn release_storage(&mut self, pos: GridPos) {
        self.reserved_storage.remove(&pos);
    }

    /// Libère un cargo
    pub fn release_cargo(&mut self, pos: GridPos) {
        self.reserved_cargo.remove(&pos);
    }

    pub fn is_rack(&self, pos: GridPos) -> bool {
        self.racks.iter().any(|r| r.contains(pos))
    }
}