extern crate sdl2;

use rand::rngs::ThreadRng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::{thread, time};

mod frame;
mod observer;
mod parameters;
mod position;
mod ray;
mod ray_trace;
mod speed;
mod sphere;
mod util;
mod vector;

use crate::observer::Observer;
use crate::parameters::Parameters;
use crate::sphere::Sphere;

mod display_ray_tracing;
use display_ray_tracing::display;
// mod display_2d;
// use display_2d::display;

fn reload_params(
    default: Option<Parameters>,
    default_observer: Option<Observer>,
) -> (Parameters, Observer) {
    let params: Parameters = Parameters::get_from_json(default);

    let mut observer: Observer = Observer::default(&params);
    match default_observer {
        Some(obs) => {
            observer.body.pos = obs.body.pos;
            observer.hor_angle = obs.hor_angle;
            observer.ver_angle = obs.ver_angle;
        }
        _ => {}
    };

    return (params, observer);
}

fn generate_sphere_vector(params: &Parameters, rng: &mut ThreadRng) -> Vec<Sphere> {
    let mut sphere_vector: Vec<Sphere> = vec![];
    Sphere::fill_vector_multiple_parameters(
        &mut sphere_vector,
        &params.sphere_parameters,
        &params.physics_parameters,
        rng,
    );
    return sphere_vector;
}

fn main() {
    // init RNG
    let mut rng: ThreadRng = rand::thread_rng();

    // init params, observer and sphere_vector
    let mut params: Parameters;
    let mut observer: Observer;
    (params, observer) = reload_params(Option::None, Option::None);

    let mut sphere_vector = generate_sphere_vector(&params, &mut rng);

    // init video subsystem
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // open window and convert to canvas
    let window = video_subsystem
        .window(
            "CPU Raytracing",
            ((params.ray_parameters.max_hor_value - params.ray_parameters.min_hor_value) as f64
                * params.display_scale) as u32,
            ((params.ray_parameters.max_ver_value - params.ray_parameters.min_ver_value) as f64
                * params.display_scale) as u32,
        )
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    // scale and viewport of the canvas
    canvas
        .set_scale(params.display_scale as f32, params.display_scale as f32)
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
                } => observer.turn_ver(
                    params.observer_parameters.look_up_angle,
                    &params.observer_parameters,
                    &params.ray_parameters,
                ),
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => observer.turn_ver(
                    params.observer_parameters.look_down_angle,
                    &params.observer_parameters,
                    &params.ray_parameters,
                ),
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => observer.turn_hor(
                    params.observer_parameters.look_left_angle,
                    &params.observer_parameters,
                    &params.ray_parameters,
                ),
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => observer.turn_hor(
                    params.observer_parameters.look_right_angle,
                    &params.observer_parameters,
                    &params.ray_parameters,
                ),
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => observer.move_forward(
                    params.observer_parameters.move_forward_distance,
                    &params.observer_parameters,
                    &params.ray_parameters,
                ),
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => observer.move_forward(
                    params.observer_parameters.move_backward_distance,
                    &params.observer_parameters,
                    &params.ray_parameters,
                ),
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => observer.move_hor(
                    params.observer_parameters.move_left_distance,
                    &params.observer_parameters,
                    &params.ray_parameters,
                ),
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => observer.move_hor(
                    params.observer_parameters.move_right_distance,
                    &params.observer_parameters,
                    &params.ray_parameters,
                ),
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => observer.move_ver(
                    params.observer_parameters.move_up_distance,
                    &params.observer_parameters,
                    &params.ray_parameters,
                ),
                Event::KeyDown {
                    keycode: Some(Keycode::LShift),
                    ..
                } => observer.move_ver(
                    params.observer_parameters.move_down_distance,
                    &params.observer_parameters,
                    &params.ray_parameters,
                ),
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => observer.reset_position(&params.observer_parameters, &params.ray_parameters),
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => observer.switch_accumulation_mode(),
                Event::KeyDown {
                    keycode: Some(Keycode::LCtrl),
                    ..
                } => observer.slow_speed_mode(),
                Event::KeyUp {
                    keycode: Some(Keycode::LCtrl),
                    ..
                } => observer.normal_speed_mode(),
                Event::KeyDown {
                    keycode: Some(Keycode::V),
                    ..
                } => observer.switch_visibility(),
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => params.physics_parameters.enabled = !params.physics_parameters.enabled,
                Event::KeyDown {
                    keycode: Some(Keycode::Tab),
                    ..
                } => {
                    (params, observer) = reload_params(Option::Some(params), Option::Some(observer))
                }
                Event::KeyDown {
                    keycode: Some(Keycode::G),
                    ..
                } => sphere_vector = generate_sphere_vector(&params, &mut rng),
                _ => {}
            }
        }

        // physics
        if params.physics_parameters.enabled {
            for s in sphere_vector.iter_mut() {
                s.physics(&params.physics_parameters);
            }
        }

        // draw and refresh the canvas display
        display(
            &mut observer,
            &sphere_vector,
            &params.ray_parameters,
            &mut canvas,
        );

        // sleep between frames
        thread::sleep(time::Duration::from_millis(params.frame_period_ms));
    }
}
