pub enum PointSide {
    Right,
    On,
    Left,
}

impl PointSide {
    pub fn classify(f: f32) -> Self {
        match f {
            f if f > 0.0 => PointSide::Right,
            f if f < 0.0 => PointSide::Left,
            _ => PointSide::On,
        }
    }
}

pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl From<Vec2> for Vec2i {
    fn from(o: Vec2) -> Self {
        Vec2i {
            x: o.x as i32,
            y: o.y as i32,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn dot(&mut self, o: Vec2) {
        self.x *= o.x;
        self.y *= o.y;
    }

    fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalize(&mut self) {
        let len = self.len();
        self.x /= len;
        self.y /= len;
    }

    fn rot(&mut self, angle: f32) {
        let x = self.x;
        let y = self.y;
        self.x = x * angle.cos() - y * angle.sin();
        self.y = x * angle.sin() + y * angle.cos();
    }

    fn point_side(&self, a: Vec2, b: Vec2) -> PointSide {
        let val = (b.x - a.x) * (self.y - a.y) - (b.y - a.y) * (self.x - a.x);
        PointSide::classify(val)
    }
}

impl From<Vec2i> for Vec2 {
    fn from(o: Vec2i) -> Self {
        Vec2 {
            x: o.x as f32,
            y: o.y as f32,
        }
    }
}
