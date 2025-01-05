use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum, Sdl, VideoSubsystem};

use doom_rs::modules::{self, Camera, Keystate, State, Vec2, SCREEN_HEIGHT, SCREEN_WIDTH};

const ROTSPEED: f32 = 3.0 * 0.016;
const MOVESPEED: f32 = 3.0 * 0.016;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context: Sdl = sdl2::init()?;
    let video_subsystem: VideoSubsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("DOOM-RS", 1280, 780)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::ARGB8888, SCREEN_WIDTH, SCREEN_HEIGHT)
        .unwrap();

    let mut dir: Vec2 = Vec2::new(-1.0, 0.1);
    dir.normalize();

    let camera = Camera::builder()
        .pos(Vec2::new(2.0, 2.0))
        .dir(dir)
        .plane(Vec2::new(0.0, 0.66))
        .build();

    let mut state = State::builder()
        .canvas(canvas)
        .texture(texture)
        .camera(camera)
        .build();

    let mut keystate = Keystate::default();
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            };

            keystate.handle_sdl_event(&event);
        }

        if keystate.is_down(Keycode::Left) {
            state.camera.rotate(ROTSPEED);
        }

        if keystate.is_down(Keycode::Right) {
            state.camera.rotate(-ROTSPEED);
        }

        if keystate.is_down(Keycode::Up) {
            state.camera.pos.x += state.camera.dir.x * MOVESPEED;
            state.camera.pos.y += state.camera.dir.y * MOVESPEED;
        }

        if keystate.is_down(Keycode::Down) {
            state.camera.pos.x -= state.camera.dir.x * MOVESPEED;
            state.camera.pos.y -= state.camera.dir.y * MOVESPEED;
        }

        state.clear_pixels();

        modules::render(&mut state);

        state.update_texture();
        state.render();
    }

    std::process::exit(0)
}
