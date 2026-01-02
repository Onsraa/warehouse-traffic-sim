// === GRID ===
pub const GRID_WIDTH: u32 = 50;
pub const GRID_HEIGHT: u32 = 50;
pub const CELL_SIZE: f32 = 1.0;

// === SIMULATION ===
pub const TICK_RATE_HZ: f64 = 20.0;
pub const TICK_DELTA: f32 = 1.0 / TICK_RATE_HZ as f32;

// === ROBOT ===
pub const ROBOT_COUNT: u32 = 100;
pub const ROBOT_MAX_VELOCITY: f32 = 3.0;      // m/s
pub const ROBOT_ACCELERATION: f32 = 2.0;      // m/s²
pub const ROBOT_DECELERATION: f32 = 3.0;      // m/s²

// === RESERVATION ===
pub const RESERVATION_DURATION_TICKS: u64 = 10;