use rayon::prelude::*;
use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::{observer::Observer, parameters::Parameters, ray::Ray, sphere::Sphere};

fn add_fog(factor: f64, ray_length: f64, color: Color, parameters: &Parameters) -> Color {
    let mult = (1.0
        - ((factor * ray_length * parameters.fog_factor)
            / parameters.observer_look_vector_distance))
        .max(parameters.min_pixel_factor);

    return Color::RGB(
        (color.r as f64 * mult) as u8,
        (color.g as f64 * mult) as u8,
        (color.b as f64 * mult) as u8,
    );
}

pub fn get_average_color(color_vector: &Vec<&Color>, importance_factor: f64) -> Color {
    let mut r: f64 = 0.0;
    let mut g: f64 = 0.0;
    let mut b: f64 = 0.0;
    let mut ratio: f64 = 1.0;
    let mut total: f64 = 0.0;

    for c in color_vector {
        r += c.r as f64 * ratio;
        g += c.g as f64 * ratio;
        b += c.b as f64 * ratio;
        total += ratio;
        ratio *= importance_factor;
    }

    return Color::RGB(
        ((r / total) as u128) as u8,
        ((g / total) as u128) as u8,
        ((b / total) as u128) as u8,
    );
}
// let mut color_vector: Vec<&Color> = Vec::new()
struct RayCollision {
    pub factor: f64,
    pub length: f64,
    pub color_vector: Vec<Color>,
}

impl RayCollision {
    fn default() -> RayCollision {
        return RayCollision {
            factor: 0.,
            length: 0.,
            color_vector: Vec::new(),
        };
    }
}

fn get_ray_collision(
    sphere_vector: &Vec<Sphere>,
    parameters: &Parameters,
    ray: &Ray,
    result: &mut RayCollision,
    remaining_bounces: u32,
) {
    if remaining_bounces > 0 {
        let mut collision: Option<(f64, &Sphere)> = ray.find_collision(sphere_vector);
        let mut bounce_factor: f64;
        let mut bounce_sphere: &Sphere;
        let mut bounce_ray: Ray;

        match collision {
            None => {
                result.color_vector.push(parameters.background_color);
            }
            Some((factor, sphere)) => {
                result.color_vector.push(sphere.color);

                bounce_ray = ray.clone(); // TODO remove
                bounce_factor = factor;
                bounce_sphere = sphere;

                if remaining_bounces > 0 {
                    bounce_ray = bounce_ray.get_reflection(bounce_factor, bounce_sphere);

                    get_ray_collision(
                        sphere_vector,
                        parameters,
                        &bounce_ray,
                        result,
                        remaining_bounces - 1,
                    );

                    // collision = bounce_ray.find_collision(sphere_vector);

                    // match collision {
                    //     None => {
                    //         result.color_vector.push(&parameters.background_color);
                    //         break;
                    //     }
                    //     Some((factor, sphere)) => {
                    //         color_vector.push(&sphere.color);
                    //         bounce_factor = factor;
                    //         bounce_sphere = sphere;
                    //     }
                    // };
                }

                // result.replace(RayCollision {
                //     factor: factor,
                //     length: ray.length,
                //     color_vector: color_vector,
                // });
                // return Option::Some(RayCollision {
                //     factor: factor,
                //     length: ray.length,
                //     color_vector: &color_vector
                // });
            }
        };
    }
}

pub fn display(
    observer: &Observer,
    sphere_vector: &Vec<Sphere>,
    parameters: &Parameters,
    canvas: &mut Canvas<Window>,
) {
    canvas.set_draw_color(parameters.background_color);
    canvas.clear();

    let rays_slice: &[Ray] = &observer.rays;

    let mut ray_collisions: Vec<RayCollision> = Vec::with_capacity(rays_slice.len());
    
    for i in 0..ray_collisions.len() {
        ray_collisions[i] = RayCollision::default();
    }

    rays_slice.par_iter().enumerate().for_each(|(index, ray)| {
        let col: &mut RayCollision = &mut ray_collisions[index];
        get_ray_collision(
            sphere_vector,
            parameters,
            ray,
            col,
            parameters.ray_bounce_count,
        );
    });

    // canvas.set_draw_color(add_fog(
    //     factor,
    //     ray.length,
    //     get_average_color(color_vector, parameters.ray_bounce_color_reflection_factor),
    //     parameters,
    // ));
    // canvas
    //     .draw_point(Point::new(ray.x_value, ray.y_value))
    //     .unwrap();

    canvas.present();
}
