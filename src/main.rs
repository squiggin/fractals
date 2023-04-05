mod complex;
mod fractal;

extern crate sdl2;

use fractal::draw_fractal;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Fractal Window", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // The rest of the game loop goes here...
        canvas.set_draw_color(Color::RGB(255, 255, 0));

        let before = std::time::SystemTime::now();
        let points = draw_fractal(WIDTH as usize, HEIGHT as usize);
        let after = std::time::SystemTime::now();
        println!(
            "Frame generation time: {}",
            after.duration_since(before).unwrap().as_secs_f32()
        );

        canvas.draw_points(points.as_slice()).unwrap();
        canvas.present();
    }
}
