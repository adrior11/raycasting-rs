use std::f32::consts::{PI, TAU};

use super::{
    consts::{HFOV, PI_2, PI_4, SCREEN_WIDTH},
    vec2::Vec2,
};

pub fn deg_to_rad(d: f32) -> f32 {
    d * PI / 180.0
}

pub fn rad_to_deg(r: f32) -> f32 {
    r * 180.0 / PI
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    x.max(min).min(max)
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn remap(x: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    out_min + (x - in_min) * (out_max - out_min) / (in_max - in_min)
}

pub fn sign(x: f32) -> f32 {
    match x {
        x if x > 0.0 => 1.0,
        x if x < 0.0 => -1.0,
        _ => 0.0,
    }
}

pub fn interset_segments(a0: Vec2, a1: Vec2, b0: Vec2, b1: Vec2) -> Option<Vec2> {
    let d = ((a0.x - a1.x) * (b0.y - b1.y)) - ((a0.y - a1.y) * (b0.x - b1.x));

    if d.abs() < 0.000001 {
        return None;
    }

    let t = (((a0.x - b0.x) * (b0.y - b1.y)) - ((a0.y - b0.y) * (b0.x - b1.x))) / d;
    let u = (((a0.x - b0.x) * (a0.y - a1.y)) - ((a0.y - b0.y) * (a0.x - a1.x))) / d;

    if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
        Some(Vec2::new(
            a0.x + t * (a1.x - a0.x),
            a0.y + t * (a1.y - a0.y),
        ))
    } else {
        None
    }
}

pub fn abgr_mul(col: u32, a: u32) -> u32 {
    let br = ((col & 0xFF00FF) * a) >> 8;
    let g = ((col & 0x00FF00) * a) >> 8;
    0xFF000000 | (br & 0xFF00FF) | (g & 0x00FF00)
}

pub fn screen_angle_to_x(angle: f32) -> u32 {
    ((SCREEN_WIDTH / 2) as f32 * (1. - (angle + (HFOV / 2.) * PI_2 - PI_4))) as u32
}

pub fn normalize_angle(angle: f32) -> f32 {
    angle - (TAU * ((angle + PI) / TAU).floor())
}

pub fn ver_line(x: u32, y0: u32, y1: u32, color: u32) {
    for y in y0..=y1 {
        // FIX:
        // &STATE.pixels[y * SCREEN_WIDTH + x] = color;
    }
}
