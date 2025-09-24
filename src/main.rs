extern crate sdl2;

use rand::rngs::ThreadRng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;
use std::{thread, time};

mod frame;
mod object;
mod observer;
mod parameters;
mod polygon;
mod position;
mod ray;
mod ray_trace;
mod speed;
mod sphere;
mod surface_type;
mod util;
mod vector;

use crate::object::Object;
use crate::observer::Observer;
use crate::parameters::Parameters;
use crate::polygon::Polygon;
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

    observer.generate_rays(&params);

    return (params, observer);
}

fn resize_canvas(params: &Parameters, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    // scale and viewport of the canvas
    canvas
        .set_scale(
            (params.display_width / (params.h_rays as f64)) as f32,
            (params.display_height / (params.v_rays as f64)) as f32,
        )
        .unwrap();
}

fn generate_object_vector(params: &Parameters, rng: &mut ThreadRng) -> Vec<Object> {
    let mut object_vector: Vec<Object> = vec![];
    Sphere::fill_vector_multiple_parameters(
        &mut object_vector,
        &params.sphere_parameters,
        &params.physics_parameters,
        rng,
    );
    Polygon::fill_vector_multiple_parameters(&mut object_vector);
    return object_vector;
}

fn main() {
    // init RNG
    let mut rng: ThreadRng = rand::thread_rng();

    // init params, observer and sphere_vector
    let mut params: Parameters;
    let mut observer: Observer;

    (params, observer) = reload_params(Option::None, Option::None);

    let mut object_vector = generate_object_vector(&params, &mut rng);

    // init video subsystem
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // open window and convert to canvas
    let window: sdl2::video::Window = video_subsystem
        .window(
            "CPU Raytracing",
            params.display_width as u32,
            params.display_height as u32,
        )
        .build()
        .unwrap();

    let mut canvas: sdl2::render::Canvas<sdl2::video::Window> =
        window.into_canvas().build().unwrap();
    resize_canvas(&params, &mut canvas);

    let mut event_pump: sdl2::EventPump = sdl_context.event_pump().unwrap();

    let mut pressed_keys: HashMap<Keycode, bool> = HashMap::new();

    // main loop
    'main_loop: loop {
        // check for key presses... Without this the window is unresponsive
        for event in event_pump.poll_iter() {
            match event {
                // TODO - Make this cleaner
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                Event::KeyDown {
                    keycode: Some(code),
                    ..
                } => {
                    pressed_keys.insert(code, true);
                    match code {
                        Keycode::R => {
                            observer.reset_position(&params);
                            break;
                        }
                        Keycode::Return => {
                            observer.switch_accumulation_mode();
                            break;
                        }
                        Keycode::KpMinus => {
                            observer.slow_speed_mode();
                            break;
                        }
                        Keycode::KpPlus => {
                            observer.normal_speed_mode();
                            break;
                        }
                        Keycode::V => {
                            observer.switch_visibility();
                            break;
                        }
                        Keycode::P => {
                            params.physics_parameters.enabled = !params.physics_parameters.enabled;
                            break;
                        }
                        Keycode::Tab => {
                            (params, observer) =
                                reload_params(Option::Some(params), Option::Some(observer));
                            // resize_canvas(&params, &mut canvas);
                            break;
                        }
                        Keycode::G => {
                            object_vector = generate_object_vector(&params, &mut rng);
                            break;
                        }
                        _ => {}
                    }
                }
                Event::KeyUp {
                    keycode: Some(code),
                    ..
                } => {
                    pressed_keys.insert(code, false);
                }
                _ => {}
            }
        }
        for (key, pressed) in &pressed_keys {
            if *pressed {
                match key {
                    Keycode::Up => {
                        observer.turn_ver(params.observer_parameters.look_up_angle, &params)
                    }
                    Keycode::Down => {
                        observer.turn_ver(params.observer_parameters.look_down_angle, &params)
                    }
                    Keycode::Left => {
                        observer.turn_hor(params.observer_parameters.look_left_angle, &params)
                    }
                    Keycode::Right => {
                        observer.turn_hor(params.observer_parameters.look_right_angle, &params)
                    }
                    Keycode::W => observer
                        .move_forward(params.observer_parameters.move_forward_distance, &params),
                    Keycode::S => observer
                        .move_forward(params.observer_parameters.move_backward_distance, &params),
                    Keycode::A => {
                        observer.move_hor(params.observer_parameters.move_left_distance, &params)
                    }
                    Keycode::D => {
                        observer.move_hor(params.observer_parameters.move_right_distance, &params)
                    }
                    Keycode::Space => {
                        observer.move_ver(params.observer_parameters.move_up_distance, &params)
                    }
                    Keycode::LShift => {
                        observer.move_ver(params.observer_parameters.move_down_distance, &params)
                    }
                    _ => {}
                }
            }
        }

        // physics
        if params.physics_parameters.enabled {
            for s in object_vector.iter_mut() {
                match s {
                    Object::Sphere(s) => s.physics(&params.physics_parameters),
                    Object::Polygon(_p) => {}
                }
            }
        }

        // draw and refresh the canvas display
        display(
            &mut observer,
            &object_vector,
            &params.ray_parameters,
            &mut canvas,
        );

        // sleep between frames
        thread::sleep(time::Duration::from_millis(params.frame_period_ms));
    }
}
