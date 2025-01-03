use std::f32::consts::PI;

pub const PI_2: f32 = PI / 2.;
pub const PI_4: f32 = PI / 4.;

pub const SCREEN_WIDTH: u32 = 384;
pub const SCREEN_HEIGHT: u32 = 216;

pub const EYE_Z: f32 = 1.65;
pub const HFOV: f32 = 90.0 * PI / 180.0;
pub const VFOV: f32 = 0.5;

pub const MAP_SIZE: usize = 8;
pub const MAP_DATA: [[u32; MAP_SIZE]; MAP_SIZE] = [
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 3, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 2, 0, 4, 4, 0, 1],
    [1, 0, 0, 0, 4, 0, 0, 1],
    [1, 0, 3, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
];
