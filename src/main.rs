extern crate sdl2;
use rand::rngs::ThreadRng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::{thread, time};

mod observer;
mod parameters;
mod position;
mod ray;
mod sphere;

use crate::observer::Observer;
use crate::parameters::Parameters;
use crate::sphere::Sphere;

mod display_ray_tracing;
use display_ray_tracing::display;
// mod display_2d;
// use display_2d::display;

fn main() {
    // init params
    let params: Parameters = Parameters::default();

    // init RNG
    let mut rng: ThreadRng = rand::thread_rng();

    // init sphere vector
    let mut sphere_vector = Sphere::good_ol_vector(&params, &mut rng);

    // init observer
    let mut observer = Observer::default(&params);

    // init video subsystem
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // open window and convert to canvas
    let window = video_subsystem
        .window(
            "Example",
            ((params.max_hor_ray_value - params.min_hor_ray_value) as f32 * params.display_scale)
                as u32,
            ((params.max_ver_ray_value - params.min_ver_ray_value) as f32 * params.display_scale)
                as u32,
        )
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    // scale and viewport of the canvas
    canvas
        .set_scale(params.display_scale, params.display_scale)
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // main loop
    'main_loop: loop {
        // check for key presses... Without this the window is unresponsive
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => observer.turn_ver(params.observer_look_up_angle, &params),
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => observer.turn_ver(params.observer_look_down_angle, &params),
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => observer.turn_hor(params.observer_look_left_angle, &params),
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => observer.turn_hor(params.observer_look_right_angle, &params),
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => observer.move_forward(params.observer_move_forward_distance, &params),
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => observer.move_forward(params.observer_move_backward_distance, &params),
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => observer.move_hor(params.observer_move_left_distance, &params),
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => observer.move_hor(params.observer_move_right_distance, &params),
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => observer.move_ver(params.observer_move_up_distance, &params),
                Event::KeyDown {
                    keycode: Some(Keycode::LShift),
                    ..
                } => observer.move_ver(params.observer_move_down_distance, &params),
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => observer = Observer::default(&params),
                _ => {}
            }
        }

        // physics
        if params.physics {
            for s in sphere_vector.iter_mut() {
                s.physics(&params);
            }
        }

        // draw and refresh the canvas display
        display(&observer, &sphere_vector, &params, &mut canvas);

        // sleep between frames
        thread::sleep(time::Duration::from_millis(params.frame_period_ms));
    }
}
