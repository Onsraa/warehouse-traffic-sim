use bevy::prelude::*;
use super::GridPos;
use crate::constants::{CELL_SIZE, GRID_HEIGHT, GRID_WIDTH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CellType {
    #[default]
    Floor,
    Rack,
    Blocked,
}

impl CellType {
    #[inline]
    pub fn is_passable(&self) -> bool {
        matches!(self, Self::Floor)
    }
}

#[derive(Resource)]
pub struct WarehouseGrid {
    width: u32,
    height: u32,
    cells: Vec<CellType>,
}

impl Default for WarehouseGrid {
    fn default() -> Self {
        Self::new(GRID_WIDTH, GRID_HEIGHT)
    }
}

impl WarehouseGrid {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            cells: vec![CellType::Floor; size],
        }
    }

    #[inline]
    fn index(&self, pos: GridPos) -> Option<usize> {
        if pos.x < 0 || pos.y < 0 {
            return None;
        }
        let (x, y) = (pos.x as u32, pos.y as u32);
        if x >= self.width || y >= self.height {
            return None;
        }
        Some((y * self.width + x) as usize)
    }

    pub fn get(&self, pos: GridPos) -> Option<CellType> {
        self.index(pos).map(|i| self.cells[i])
    }

    pub fn set(&mut self, pos: GridPos, cell: CellType) {
        if let Some(i) = self.index(pos) {
            self.cells[i] = cell;
        }
    }

    pub fn is_passable(&self, pos: GridPos) -> bool {
        self.get(pos).map(|c| c.is_passable()).unwrap_or(false)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    pub fn grid_to_world(&self, pos: GridPos) -> (f32, f32) {
        (
            pos.x as f32 * CELL_SIZE + CELL_SIZE * 0.5,
            pos.y as f32 * CELL_SIZE + CELL_SIZE * 0.5,
        )
    }
}