extern crate sdl2;

use crate::complex::Complex;
use sdl2::rect::Point;
use std::thread;

const NTHREADS: usize = 8;

fn map_to_range(num: f32, from: (f32, f32), to: (f32, f32)) -> f32 {
    (num - from.0) * (to.1 - to.0) / (from.1 - from.0) + to.0
}

fn is_in_fractal(x: f32, y: f32) -> u8 {
    let point = Complex::new(x, y);

    let mut z = Complex::new(0.0, 0.0);
    let mut i = 0;
    while i < 200 && z.squared_abs() < 4.0 {
        z = z.square() + point;
        i += 1;
    }

    i
}

pub fn draw_fractal(width: usize, height: usize) -> Vec<(Point, u8)> {
    let res = 1;

    let mut thread_handlers = Vec::new();
    for i in 0..NTHREADS {
        thread_handlers.push(thread::spawn(move || {
            let mut points = Vec::new();

            // Vertical paritioning of points for multithreading
            for _y in (0..height).step_by(res) {
                for _x in ((i * (width / NTHREADS))..((i + 1) * (width / NTHREADS))).step_by(res) {
                    let x = map_to_range(_x as f32, (0.0, width as f32), (-2.00, 0.47));
                    let y = map_to_range((height - _y) as f32, (0.0, height as f32), (-1.12, 1.12));
                    points.push((Point::new(_x as i32, (height - _y) as i32), is_in_fractal(x, y)));
                }
            }

            points
        }));
    }

    let mut points = Vec::new();
    for handler in thread_handlers {
        let mut sub_points = handler.join().unwrap();
        points.append(&mut sub_points);
    }

    points
}

