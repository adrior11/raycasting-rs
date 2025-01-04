use bon::Builder;
use sdl2::{
    render::{Canvas, Texture},
    video::Window,
};

use super::{
    camera::Camera,
    consts::{SCREEN_HEIGHT, SCREEN_WIDTH},
};

#[derive(Builder)]
pub struct State<'a> {
    pub canvas: Canvas<Window>,
    pub texture: Texture<'a>,
    #[builder(default = vec![0u8; (SCREEN_WIDTH * SCREEN_HEIGHT * 4) as usize])]
    pub pixels: Vec<u8>,
    pub camera: Camera,
}

impl<'a> State<'a> {
    pub fn clear_pixels(&mut self) {
        self.pixels.fill(0);
    }

    pub fn update_texture(&mut self) {
        self.texture
            .update(None, &self.pixels.clone(), SCREEN_WIDTH as usize * 4)
            .unwrap();
    }

    pub fn render(&mut self) {
        self.canvas.clear();
        self.canvas
            .copy_ex(&self.texture, None, None, 0.0, None, false, true)
            .unwrap();
        self.canvas.present();
    }
}
