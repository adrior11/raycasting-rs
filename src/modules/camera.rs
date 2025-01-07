use bon::Builder;

use super::{
    consts::MAP_SIZE,
    util,
    vec2::{Vec2, Vec2i},
};

const CAMERA_RADIUS: f32 = 0.1;

#[derive(Builder)]
pub struct Camera {
    pub pos: Vec2,
    pub dir: Vec2,
    pub plane: Vec2,
}

impl Camera {
    /// Rotates the camera by radians around the Z-axis.
    pub fn rotate(&mut self, rad: f32) {
        let d = self.dir;
        let p = self.plane;

        // Rotate direction
        self.dir.x = d.x * rad.cos() - d.y * rad.sin();
        self.dir.y = d.x * rad.sin() + d.y * rad.cos();

        // Rotate plane
        self.plane.x = p.x * rad.cos() - p.y * rad.sin();
        self.plane.y = p.x * rad.sin() + p.y * rad.cos();
    }

    /// Utility function to attempt movement of the camera.
    pub fn try_move(&mut self, move_dir: Vec2, move_speed: f32) {
        // Compute new position
        let mut new_pos = self.pos + move_dir * move_speed;

        // Determine the region of tiles to check
        let min_tile_x = (new_pos.x - 1.0).floor().max(0.0) as i32;
        let max_tile_x = (new_pos.x + 1.0).ceil().min(MAP_SIZE as f32 - 1.0) as i32;
        let min_tile_y = (new_pos.y - 1.0).floor().max(0.0) as i32;
        let max_tile_y = (new_pos.y + 1.0).ceil().min(MAP_SIZE as f32 - 1.0) as i32;

        // For each tile in that region, if it's a wall, do bounding-box collision
        for tile_y in min_tile_y..=max_tile_y {
            for tile_x in min_tile_x..=max_tile_x {
                if util::get_tile_at(&Vec2i::new(tile_x, tile_y)) != 0 {
                    let tile_left = tile_x as f32;
                    let tile_right = tile_left + 1.0;
                    let tile_top = tile_y as f32;
                    let tile_bottom = tile_top + 1.0;

                    // Find the closest point on the tile box to new_pos
                    let closest_x = new_pos.x.clamp(tile_left, tile_right);
                    let closest_y = new_pos.y.clamp(tile_top, tile_bottom);

                    // Distance from that closest point to the camera's center
                    let mut dist = Vec2::new(new_pos.x - closest_x, new_pos.y - closest_y);

                    // Check if camera intersects the tile
                    if dist.square() < CAMERA_RADIUS * CAMERA_RADIUS {
                        let dist_len = dist.len();
                        if dist_len != 0.0 {
                            // Penetration amount
                            let pen = CAMERA_RADIUS - dist_len;

                            // Normalize to get the push direction
                            dist.normalize();

                            // Push new_pos out of the wall tile
                            new_pos += dist * pen;
                        }
                    }
                }
            }
        }

        self.pos = new_pos;
    }
}
