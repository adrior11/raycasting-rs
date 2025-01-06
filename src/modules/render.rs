use core::f32;

use super::{
    consts::{CAMERA_LUT, SCREEN_HEIGHT, SCREEN_WIDTH, WALL_DIM_FACTOR},
    state::State,
    util,
    vec2::{Vec2, Vec2i},
};

/// Stores the result of a DDA raycast check.
#[derive(Default)]
struct Hit {
    val: i32,
    side: i32,
}

/// Main rendering function to perform raycasting and draw vertical strips.
pub fn render(state: &mut State) {
    for x in 0..SCREEN_WIDTH {
        // x coordinate in camera space from [-1, 1]
        let xcam = CAMERA_LUT[x as usize];

        // Compute ray direction
        let dir = Vec2::new(
            state.camera.dir.x + state.camera.plane.x * xcam,
            state.camera.dir.y + state.camera.plane.y * xcam,
        );

        let pos = state.camera.pos;
        let mut ipos = Vec2i::from(pos);

        // Calculate the length of ray from one x or y side to the next
        let delta_dist = Vec2::new(util::safe_inv_abs(dir.x), util::safe_inv_abs(dir.y));

        let step = Vec2i::new(util::isign(dir.x), util::isign(dir.y));
        let mut side_dist = Vec2::new(
            delta_dist.x
                * (match dir.x {
                    val if val < 0.0 => pos.x - ipos.x as f32,
                    _ => ipos.x as f32 + 1.0 - pos.x,
                }),
            delta_dist.y
                * (match dir.y {
                    val if val < 0.0 => pos.y - ipos.y as f32,
                    _ => ipos.y as f32 + 1.0 - pos.y,
                }),
        );

        let mut hit = Hit::default();

        // Perform DDA
        while hit.val == 0 {
            // Jump to next map tile in X direction or Y direction
            if side_dist.x < side_dist.y {
                side_dist.x += delta_dist.x;
                ipos.x += step.x;
                hit.side = 0;
            } else {
                side_dist.y += delta_dist.y;
                ipos.y += step.y;
                hit.side = 1;
            }

            hit.val = util::get_tile_at(&ipos);
        }

        // Determine color based on hit value
        let mut color: u32 = util::get_wall_color(hit.val);

        // If hit was in y-direction (hit.side == 1), dim the color
        if hit.side == 1 {
            color = util::dim_color(color, WALL_DIM_FACTOR);
        }

        // Calculate the perpendicular distance for fish-eye correction
        let perp_dist = if hit.side == 0 {
            side_dist.x - delta_dist.x
        } else {
            side_dist.y - delta_dist.y
        };

        // Calculate the height of the wall
        let h = (SCREEN_HEIGHT as f32 / perp_dist) as i32;

        // Compute top/bottom positions of the line
        let y0 = ((SCREEN_HEIGHT / 2) as i32 - (h / 2)).max(0);
        let y1 = ((SCREEN_HEIGHT / 2) as i32 + (h / 2)).min(SCREEN_HEIGHT as i32 - 1);

        // Draw the vertical line
        ver_line(state, x, y0, y1, color);
    }
}

/// Draws a vertical line on the screen.
///
/// # Parameters
/// - `state`: Mutable reference to the game/application state (for the pixel buffer).
/// - `x`: X-coordinate on the screen where the line is drawn.
/// - `y0`: Start Y-coordinate for the line (top).
/// - `y1`: End Y-coordinate for the line (bottom).
/// - `color`: 0xAARRGGBB color to use for the line.
#[allow(clippy::identity_op)]
#[inline(always)]
fn ver_line(state: &mut State, x: u32, y0: i32, y1: i32, color: u32) {
    let pixels_ptr = state.pixels.as_mut_ptr();

    // Decompose the color (0xAARRGGBB) into separate bytes (ARGB8888 -> RGBA in memory).
    let a = ((color >> 24) & 0xFF) as u8;
    let r = ((color >> 16) & 0xFF) as u8;
    let g = ((color >> 8) & 0xFF) as u8;
    let b = ((color >> 0) & 0xFF) as u8;

    // Clamp the drawing range to the screen boundaries
    let y_start = y0.max(0) as usize;
    let y_end = y1.min((SCREEN_HEIGHT - 1) as i32) as usize;

    // Convert pixel index to a byte offset in the u8 buffer:
    // each pixel is 4 bytes, so multiply by 4.
    let mut offset = (y_start * SCREEN_WIDTH as usize + x as usize) * 4;
    let stride = SCREEN_WIDTH as usize * 4;

    for _ in y_start..=y_end {
        // Write precomputed channels
        unsafe {
            *pixels_ptr.add(offset + 0) = r;
            *pixels_ptr.add(offset + 1) = g;
            *pixels_ptr.add(offset + 2) = b;
            *pixels_ptr.add(offset + 3) = a;
        }

        // Move to next row
        offset += stride;
    }
}
