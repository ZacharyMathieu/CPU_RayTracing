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

pub fn get_average_color(color_vector: &Vec<Color>, importance_factor: f64) -> Color {
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
    pub ray: Ray,
    pub factor: f64,
    pub length: f64,
    pub color_vector: Vec<Color>,
}

impl RayCollision {
    fn default(ray: Ray) -> RayCollision {
        return RayCollision {
            ray: ray,
            factor: 0.,
            length: 0.,
            color_vector: Vec::new(),
        };
    }
}

fn get_ray_collision(
    sphere_vector: &Vec<Sphere>,
    parameters: &Parameters,
    result: &mut RayCollision,
    remaining_bounces: u32,
) {
    if remaining_bounces > 0 {
        let ray: &Ray = &result.ray;
        let mut collision: Option<(f64, &Sphere)> = ray.find_collision(sphere_vector);
        let mut bounce_factor: f64;
        let mut bounce_sphere: &Sphere;

        match collision {
            None => {
                result.color_vector.push(parameters.background_color);
            }
            Some((factor, sphere)) => {
                result.color_vector.push(sphere.color);
                result.factor = factor;
                result.length = result.ray.length;

                if remaining_bounces > 0 {
                    let bounce_ray: Ray = result.ray.get_reflection(factor, sphere);
                    result.ray = bounce_ray;

                    get_ray_collision(sphere_vector, parameters, result, remaining_bounces - 1);
                }
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

    let mut ray_collisions: Vec<RayCollision> = Vec::new();

    rays_slice.iter().for_each(|ray| {
        ray_collisions.push(RayCollision::default(*ray));
    });

    ray_collisions.iter_mut().for_each(|collision| {
        get_ray_collision(
            sphere_vector,
            parameters,
            collision,
            parameters.ray_bounce_count,
        );
    });

    ray_collisions.iter().for_each(|collision| {
        canvas.set_draw_color(add_fog(
            collision.factor,
            collision.ray.length,
            get_average_color(
                &collision.color_vector,
                parameters.ray_bounce_color_reflection_factor,
            ),
            parameters,
        ));
        canvas
            .draw_point(Point::new(collision.ray.x_value, collision.ray.y_value))
            .unwrap();
    });

    canvas.present();
}
