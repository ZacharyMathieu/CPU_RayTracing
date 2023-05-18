extern crate sdl2;
use position::Position;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::{thread, time};

mod line;
mod parameters;
mod position;
mod sphere;

use crate::parameters::Parameters;
use crate::sphere::Sphere;

mod display_raytracing;
use display_raytracing::display;
// mod display_2d;
// use display_2d::display;

fn main() {
    // init params
    let params: Parameters = Parameters::default();

    // init sphere vector
    let mut sphere_vector = Sphere::good_ol_vector(params.sphere_count);

    // init observer
    let observer = Sphere {
        p: Position {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        v_x: 0.0,
        v_y: 0.0,
        radius: 0.0,
        color: Color::RGB(0, 0, 0),
    };

    // init video subsystem
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // open window and convert to canvas
    let window = video_subsystem
        .window("Example", params.w, params.h)
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // main loop
    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                _ => {}
            }
        }

        for s in sphere_vector.iter_mut() {
            s.physics(&params);
        }

        display(&observer, &sphere_vector, &params, &mut canvas);
        thread::sleep(time::Duration::from_millis(10));
    }
}
