use bon::Builder;

use super::vec2::Vec2;

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
}
