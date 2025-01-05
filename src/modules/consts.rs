use super::util;

pub const SCREEN_WIDTH: u32 = 384;
pub const SCREEN_HEIGHT: u32 = 216;

/// Precomputed camera space values for raycasting, one per screen column.
pub const CAMERA_LUT: [f32; SCREEN_WIDTH as usize] = util::generate_camera_lut();

/// Brightness factor for walls that are perpendicular to the camera ray.
pub const WALL_DIM_FACTOR: u8 = 0xC0;

pub const MAP_SIZE: i32 = 8;
pub const MAP_DATA: [[i32; MAP_SIZE as usize]; MAP_SIZE as usize] = [
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 3, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 2, 0, 4, 4, 0, 1],
    [1, 0, 0, 0, 4, 0, 0, 1],
    [1, 0, 3, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
];

pub const COLORS: [u32; 5] = [
    0,          // No Wall
    0xFF0000FF, // Red
    0xFF00FF00, // Green
    0xFFFF0000, // Blue
    0xFFFF00FF, // Purple
];
