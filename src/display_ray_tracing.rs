use rayon::prelude::*;
use sdl2::{rect::Point, render::Canvas, video::Window};

use crate::{observer::Observer, parameters::RayParameters, ray_trace::RayTrace, sphere::Sphere};

pub fn display(
    observer: &Observer,
    sphere_vector: &Vec<Sphere>,
    ray_parameters: &RayParameters,
    canvas: &mut Canvas<Window>,
) {
    canvas.set_draw_color(ray_parameters.background_color);
    canvas.clear();

    let mut ray_traces: Vec<RayTrace> = Vec::new();

    observer.rays.iter().for_each(|ray| {
        ray_traces.push(RayTrace::new(ray, ray_parameters));
    });

    // Parallel ray casting
    ray_traces
        .par_iter_mut()
        .for_each(|trace: &mut RayTrace<'_>| {
            trace.trace(sphere_vector, ray_parameters);
        });

    // Displaying the colors
    ray_traces.iter().for_each(|trace: &RayTrace<'_>| {
        canvas.set_draw_color(trace.color);
        canvas
            .draw_point(Point::new(trace.ray.x_value, trace.ray.y_value))
            .unwrap();
    });

    canvas.present();
}
