#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

impl GridPos {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn manhattan_distance(&self, other: &Self) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }

    #[inline]
    pub fn neighbor(&self, dir: Direction) -> Self {
        let (dx, dy) = dir.to_offset();
        Self::new(self.x + dx, self.y + dy)
    }
}

/// Direction cardinale de déplacement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Direction {
    #[default]
    None,
    North,
    South,
    East,
    West,
}

impl Direction {
    pub const CARDINALS: [Self; 4] = [Self::North, Self::East, Self::South, Self::West];

    #[inline]
    pub const fn to_offset(&self) -> (i32, i32) {
        match self {
            Self::North => (0, 1),
            Self::South => (0, -1),
            Self::East => (1, 0),
            Self::West => (-1, 0),
            Self::None => (0, 0),
        }
    }

    #[inline]
    pub const fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
            Self::None => Self::None,
        }
    }
}
