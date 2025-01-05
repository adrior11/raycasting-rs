use core::f32;

use super::{consts::MAP_DATA, vec2::Vec2i};

/// Returns the map value at the given integer position.
#[inline]
pub fn get_map_value(ipos: &Vec2i) -> Result<i32, String> {
    ipos.to_map_coords()
        .map(|(y, x)| MAP_DATA[y][x])
        .ok_or_else(|| format!("Coordinates out of bounds: {:?}", ipos))
}

/// Returns the integer sign of a float value.
#[inline]
pub fn isign(x: f32) -> i32 {
    match x {
        x if x > 0.0 => 1,
        x if x < 0.0 => -1,
        _ => 0,
    }
}

/// Safely computes the inverse absolute value of `val`,
/// returning `f32::INFINITY` if `val` is very close to zero.
#[inline]
pub fn safe_inv_abs(val: f32) -> f32 {
    if val.abs() < 1e-20 {
        f32::INFINITY
    } else {
        (1.0 / val).abs()
    }
}
