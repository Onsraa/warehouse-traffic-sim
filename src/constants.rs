// === GRID ===
pub const GRID_WIDTH: u32 = 80;
pub const GRID_HEIGHT: u32 = 60;
pub const CELL_SIZE: f32 = 1.0;

// === ZONES ===
pub const SPAWN_ZONE_WIDTH: u32 = 8;
pub const CARGO_ZONE_WIDTH: u32 = 8;

// === RACKS ===
pub const RACK_LENGTH: u32 = 12;
pub const RACK_SPACING: u32 = 3;
pub const AISLE_WIDTH: u32 = 2;

// === SIMULATION ===
pub const TICK_RATE_HZ: f64 = 60.0;
pub const TICK_DELTA: f32 = 1.0 / TICK_RATE_HZ as f32;

// === ROBOT ===
pub const ROBOT_COUNT: u32 = 150;
pub const ROBOT_MAX_VELOCITY: f32 = 3.0;
pub const ROBOT_ACCELERATION: f32 = 2.0;
pub const ROBOT_DECELERATION: f32 = 3.0;

// === ACTIONS (en secondes) ===
pub const PICKUP_DURATION: f32 = 4.0;
pub const DROPOFF_DURATION: f32 = 3.0;

// === PBS CONFIG ===
pub const PBS_HORIZON_TICKS: u64 = 100;
pub const PBS_REPLAN_INTERVAL: u64 = 3;