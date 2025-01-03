use bon::Builder;
use sdl2::{render::Canvas, video::Window};

use super::{camera::Camera, vec2::Vec2};

#[derive(Builder)]
pub struct State {
    pub canvas: Canvas<Window>,
    pub pixels: Option<u32>,
    pub camera: Camera,
}

impl State {
    // TODO:
    pub fn point_in_sector(&self, p: Vec2) {}
}
