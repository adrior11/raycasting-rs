use super::{
    consts::{MAP_DATA, MAP_SIZE},
    vec2::Vec2i,
};

pub fn get_map_value(ipos: &Vec2i) -> Option<i32> {
    ipos.to_map_coords().map(|(y, x)| MAP_DATA[y][x])
}

pub fn out_of_bounds(ipos: &Vec2i) -> bool {
    ipos.x >= 0 && ipos.x < MAP_SIZE && ipos.y >= 0 && ipos.y < MAP_SIZE
}

pub fn isign(x: f32) -> i32 {
    match x {
        x if x > 0.0 => 1,
        x if x < 0.0 => -1,
        _ => 0,
    }
}
