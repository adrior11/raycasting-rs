use core::f32;

use super::{
    consts::{MAP_DATA, MAP_SIZE},
    vec2::Vec2i,
};

/// Returns the map value at the given integer position.
pub fn get_map_value(ipos: &Vec2i) -> Result<i32, String> {
    ipos.to_map_coords()
        .map(|(y, x)| MAP_DATA[y][x])
        .ok_or_else(|| format!("Coordinates out of bounds: {:?}", ipos))
}

/// Checks if the given integer coordinate is out of the map boundaries.
pub fn out_of_bounds(ipos: &Vec2i) -> bool {
    ipos.x < 0 || ipos.x >= MAP_SIZE || ipos.y < 0 || ipos.y >= MAP_SIZE
}

/// Returns the integer sign of a float value.
pub fn isign(x: f32) -> i32 {
    match x {
        x if x > 0.0 => 1,
        x if x < 0.0 => -1,
        _ => 0,
    }
}

/// Safely computes the inverse absolute value of `val`,
/// returning `f32::INFINITY` if `val` is very close to zero.
pub fn safe_inv_abs(val: f32) -> f32 {
    if val.abs() < 1e-20 {
        f32::INFINITY
    } else {
        (1.0 / val).abs()
    }
}
