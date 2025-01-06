use bon::Builder;

use super::{
    util,
    vec2::{Vec2, Vec2i},
};

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
    pub fn try_move(&mut self, dir: Vec2, move_speed: f32) {
        let new_pos = Vec2::new(
            self.pos.x + dir.x * move_speed,
            self.pos.y + dir.y * move_speed,
        );

        let new_tile = Vec2i::from(new_pos);

        if util::get_tile_at(&new_tile) == 0 {
            self.pos = new_pos;
        }
    }
}
