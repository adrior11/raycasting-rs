use core::f32;

use super::{
    consts::{COLORS, MAP_DATA},
    vec2::Vec2i,
    SCREEN_WIDTH,
};

/// Returns the integer sign of a float value.
#[inline(always)]
pub fn isign(x: f32) -> i32 {
    match x {
        x if x > 0.0 => 1,
        x if x < 0.0 => -1,
        _ => 0,
    }
}

/// Safely computes the inverse absolute value of `val`,
/// returning `f32::INFINITY` if `val` is very close to zero.
#[inline(always)]
pub fn safe_inv_abs(val: f32) -> f32 {
    if val.abs() < 1e-20 {
        f32::INFINITY
    } else {
        (1.0 / val).abs()
    }
}

/// Returns the color corresponding to the specified wall value.
#[inline(always)]
pub fn get_wall_color(val: i32) -> u32 {
    COLORS[val as usize]
}

/// Adjusts the brightness of a color in ARGB8888 format.
///
/// # Parameters
/// - `color`: 0xAARRGGBB color format
/// - `factor`: A byte between 0x00 (dark) and 0xFF (no change).
///
/// # Returns
/// - A new color with the brightness applied.
#[inline(always)]
pub fn dim_color(color: u32, factor: u8) -> u32 {
    let br = ((color & 0xFF00FF) * factor as u32) >> 8;
    let g = ((color & 0x00FF00) * factor as u32) >> 8;
    0xFF000000 | (br & 0xFF00FF) | (g & 0x00FF00)
}

/// Returns the tile value at the specified integer position.
#[inline(always)]
pub fn get_tile(ipos: &Vec2i) -> i32 {
    MAP_DATA[ipos.y as usize][ipos.x as usize]
}

pub const fn generate_camera_lut() -> [f32; SCREEN_WIDTH as usize] {
    let mut lut = [0.0; SCREEN_WIDTH as usize];
    let mut x = 0;
    while x < SCREEN_WIDTH as usize {
        lut[x] = (2.0 * (x as f32 / SCREEN_WIDTH as f32)) - 1.0;
        x += 1;
    }
    lut
}
