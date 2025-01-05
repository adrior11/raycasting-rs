use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sdl2::{pixels::PixelFormatEnum, Sdl, VideoSubsystem};

use raycasting_rs::modules::{self, Camera, State, Vec2, SCREEN_HEIGHT, SCREEN_WIDTH};

fn bench_renderer(c: &mut Criterion) {
    let mut group = c.benchmark_group("renderer_benchmark");

    let sdl_context: Sdl = sdl2::init().unwrap();
    let video_subsystem: VideoSubsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("BENCHMARK", 1280, 780)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::ARGB8888, SCREEN_WIDTH, SCREEN_HEIGHT)
        .unwrap();

    let mut dir = Vec2::new(-1.0, 0.1);
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

    group.bench_function("renderer", |b| {
        b.iter(|| {
            modules::render(black_box(&mut state));
        })
    });
}

criterion_group!(benches, bench_renderer);
criterion_main!(benches);
