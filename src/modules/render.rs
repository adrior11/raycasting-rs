use super::{
    consts::{SCREEN_HEIGHT, SCREEN_WIDTH},
    state::State,
    util,
    vec2::{Vec2, Vec2i},
};

#[derive(Default)]
struct Hit {
    val: i32,
    side: i32,
    pos: Vec2,
}

fn ver_line(state: &mut State, x: u32, y0: i32, y1: i32, color: u32) {
    let pixels = &mut state.pixels;

    let y_start = y0.max(0) as usize;
    let y_end = y1.min((SCREEN_HEIGHT - 1) as i32) as usize;

    for y in y_start..=y_end {
        // Convert pixel index to a byte offset in the u8 buffer:
        // each pixel is 4 bytes, so multiply by 4.
        let offset = ((y * SCREEN_WIDTH as usize) + x as usize) * 4;

        // Decompose the color (0xAARRGGBB) into separate bytes.
        // For ARGB8888 in memory (R, G, B, A):
        // BUG: index out of bounds: the len is 331776 but the index is 334656
        pixels[offset] = ((color >> 16) & 0xFF) as u8; // R
        pixels[offset + 1] = ((color >> 8) & 0xFF) as u8; // G
        pixels[offset + 2] = ((color) & 0xFF) as u8; // B
        pixels[offset + 3] = ((color >> 24) & 0xFF) as u8; // A
    }
}

pub fn render(state: &mut State) {
    for x in 0..SCREEN_WIDTH {
        // x coordinate in space from [-1, 1]
        let xcam = (2.0 * (x as f32 / SCREEN_WIDTH as f32)) - 1.0;

        let dir = Vec2::new(
            state.camera.dir.x + state.camera.plane.x * xcam,
            state.camera.dir.y + state.camera.plane.y * xcam,
        );

        let pos = state.camera.pos;
        let mut ipos = Vec2i::from(pos);

        let delta_dist = Vec2::new(
            match dir.x.abs() {
                val if val < 1e-20 => f32::INFINITY,
                val => (1.0 / val).abs(),
            },
            match dir.y.abs() {
                val if val < 1e-20 => f32::INFINITY,
                val => (1.0 / val).abs(),
            },
        );

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

        let step = Vec2i::new(util::isign(dir.x), util::isign(dir.y));

        let mut hit = Hit::default();

        while hit.val == 0 {
            if side_dist.x < side_dist.y {
                side_dist.x += delta_dist.x;
                ipos.x += step.x;
                hit.side = 0;
            } else {
                side_dist.y += side_dist.y;
                ipos.y += step.y;
                hit.side = 1;
            }

            assert!(util::out_of_bounds(&ipos), "DDA OoB");

            hit.val = util::get_map_value(&ipos).expect("Should have checked OoB");
        }

        let mut color: u32 = match hit.val {
            1 => 0xFF0000FF,
            2 => 0xFF00FF00,
            3 => 0xFFFF0000,
            4 => 0xFFFF00FF,
            _ => unreachable!("Map is misconfigured"),
        };

        if hit.side == 1 {
            let br = ((color & 0xFF00FF) * 0xC0) >> 8;
            let g = ((color & 0x00FF00) * 0xC0) >> 8;
            color = 0xFF0000 | (br & 0xFF00FF) | (g & 0x00FF00)
        }

        hit.pos = Vec2::new(pos.x + side_dist.x, pos.y + side_dist.y);

        let perp_dist = if hit.side == 0 {
            side_dist.x - delta_dist.x
        } else {
            side_dist.y - delta_dist.y
        };

        let h = (SCREEN_HEIGHT as f32 / perp_dist) as i32;
        let y0 = ((SCREEN_HEIGHT / 2) as i32 - (h / 2)).max(0);
        let y1 = ((SCREEN_HEIGHT / 2) as i32 + (h / 2)).min(SCREEN_HEIGHT as i32 - 1);

        ver_line(state, x, y0, y1, color);
    }
}
