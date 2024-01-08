use rayon::prelude::*;
use sdl2::{rect::Point, render::Canvas, video::Window};

use crate::{observer::Observer, parameters::Parameters, ray_trace::RayTrace, sphere::Sphere};

pub fn display(
    observer: &Observer,
    sphere_vector: &Vec<Sphere>,
    parameters: &Parameters,
    canvas: &mut Canvas<Window>,
) {
    canvas.set_draw_color(parameters.background_color);
    canvas.clear();

    let mut ray_traces: Vec<RayTrace> = Vec::new();

    observer.rays.iter().for_each(|ray| {
        ray_traces.push(RayTrace::new(ray, parameters));
    });

    // Parallel ray casting
    ray_traces.par_iter_mut().for_each(|trace| {
        trace.trace(sphere_vector, parameters);
    });

    // Displaying the colors
    ray_traces.iter().for_each(|trace| {
        canvas.set_draw_color(trace.color);
        canvas
            .draw_point(Point::new(trace.ray.x_value, trace.ray.y_value))
            .unwrap();
    });

    canvas.present();
}
