use super::consts::MAP_SIZE;

#[derive(Debug)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn to_map_coords(&self) -> Option<(usize, usize)> {
        if self.x >= 0 && self.x < MAP_SIZE && self.y >= 0 && self.y < MAP_SIZE {
            Some((self.y as usize, self.x as usize))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalize(&mut self) {
        let len = self.len();
        self.x /= len;
        self.y /= len;
    }
}

impl From<Vec2> for Vec2i {
    fn from(o: Vec2) -> Self {
        Vec2i {
            x: o.x as i32,
            y: o.y as i32,
        }
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
