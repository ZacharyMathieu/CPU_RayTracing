use rand::rngs::ThreadRng;
use sdl2::pixels::Color;

use crate::{parameters::RayParameters, ray::Ray, sphere::Sphere};

pub struct RayTrace<'a> {
    pub ray: &'a Ray,
    color_vector: Vec<Color>,
    pub color: Color,
}

impl<'a> RayTrace<'a> {
    pub fn new(ray: &'a Ray, ray_parameters: &RayParameters) -> Self {
        return RayTrace {
            ray: ray,
            color_vector: Vec::new(),
            color: ray_parameters.background_color,
        };
    }

    pub fn trace(&mut self, sphere_vector: &Vec<Sphere>, ray_parameters: &RayParameters) {
        let mut rng: ThreadRng = rand::thread_rng();
        self.trace_rec(
            self.ray,
            sphere_vector,
            ray_parameters,
            ray_parameters.bounce_count,
            &0.,
            &mut rng,
        );

        self.set_color(ray_parameters)
    }

    fn set_color(&mut self, ray_parameters: &RayParameters) {
        if self.color_vector.len() > 0 {
            self.color = get_average_color(
                &self.color_vector,
                &ray_parameters.bounce_color_reflection_factor,
            );
        }
    }

    fn trace_rec(
        &mut self,
        ray: &Ray,
        sphere_vector: &Vec<Sphere>,
        ray_parameters: &RayParameters,
        remaining_bounces: u32,
        distance: &f64,
        rng: &mut rand::prelude::ThreadRng,
    ) {
        let collision: Option<(f64, &Sphere)> = ray.find_collision(sphere_vector);

        match collision {
            None => {
                if (ray_parameters.reflect_background)
                    && (remaining_bounces > 0)
                    && (*distance > 0.)
                {
                    self.color_vector.push(ray_parameters.background_color)
                }
            }
            Some((factor, sphere)) => {
                let new_distance: f64 = distance + (ray.length * factor);

                self.color_vector.push(apply_light_factor(
                    &sphere.color,
                    &get_light_factor(&new_distance, &sphere.light_factor, ray_parameters),
                ));

                if remaining_bounces > 0 {
                    let ray_bounce = ray.get_bounce(factor, sphere, ray_parameters, rng);

                    self.trace_rec(
                        &ray_bounce,
                        sphere_vector,
                        ray_parameters,
                        remaining_bounces - 1,
                        &new_distance,
                        rng,
                    );
                }
            }
        };
    }
}

fn get_light_factor(
    length: &f64,
    sphere_light_factor: &f64,
    ray_parameters: &RayParameters,
) -> f64 {
    return if ray_parameters.fog_factor == 0. {
        1.
    } else {
        ((1. / ((length + (1. / ray_parameters.fog_factor)) * ray_parameters.fog_factor))
            * sphere_light_factor)
            .max(ray_parameters.min_pixel_factor)
            .min(1.)
    };
}

fn apply_light_factor(color: &Color, light_factor: &f64) -> Color {
    return Color::RGB(
        (color.r as f64 * light_factor) as u8,
        (color.g as f64 * light_factor) as u8,
        (color.b as f64 * light_factor) as u8,
    );
}

fn get_average_color(color_vector: &Vec<Color>, importance_factor: &f64) -> Color {
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
