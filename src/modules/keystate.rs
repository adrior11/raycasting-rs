use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;

#[derive(Default)]
pub struct Keystate {
    pressed_keys: HashSet<Keycode>,
}

impl Keystate {
    pub fn is_down(&self, key: Keycode) -> bool {
        self.pressed_keys.contains(&key)
    }

    pub fn handle_sdl_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                keycode: Some(kc),
                repeat: false,
                ..
            } => {
                self.pressed_keys.insert(*kc);
            }
            Event::KeyUp {
                keycode: Some(kc), ..
            } => {
                self.pressed_keys.remove(kc);
            }
            _ => {}
        }
    }
}
