use bon::Builder;

use super::vec2::Vec2;

#[derive(Builder)]
pub struct Camera {
    pub pos: Vec2,
    pub dir: Vec2,
    pub plane: Vec2,
}

impl Camera {
    pub fn rotate(&mut self, f: f32) {
        let d = self.dir;
        let p = self.plane;
        self.dir.x = d.x * f.cos() - d.y * f.sin();
        self.dir.y = d.x * f.sin() + d.y * f.cos();
        self.plane.x = p.x * f.cos() - p.y * f.sin();
        self.plane.y = p.x * f.sin() + p.y * f.cos();
    }
}
