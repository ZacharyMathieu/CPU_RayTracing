use rand::rngs::ThreadRng;
use sdl2::pixels::Color;

use crate::{
    parameters::RayParameters,
    polygon::Polygon,
    ray::{Collision, Ray},
    sphere::Sphere,
};

pub struct RayTrace<'a> {
    pub ray: &'a Ray,
    color_vector: Vec<(Color, f64)>,
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

    pub fn trace(
        &mut self,
        ray_parameters: &RayParameters,
        sphere_vector: &Vec<Sphere>,
        polygon_vector: &Vec<Polygon>,
        observer_bodies: &Vec<Sphere>,
    ) {
        let mut rng: ThreadRng = rand::thread_rng();
        self.trace_rec(
            self.ray,
            ray_parameters,
            sphere_vector,
            polygon_vector,
            observer_bodies,
            ray_parameters.bounce_count,
            &0.,
            &mut rng,
        );

        self.set_color(ray_parameters)
    }

    fn set_color(&mut self, ray_parameters: &RayParameters) {
        if self.color_vector.len() > 0 {
            self.color = self.get_average_color(&ray_parameters.bounce_color_reflection_factor);
        }
    }

    fn trace_rec(
        &mut self,
        ray: &Ray,
        ray_parameters: &RayParameters,
        sphere_vector: &Vec<Sphere>,
        polygon_vector: &Vec<Polygon>,
        observer_bodies: &Vec<Sphere>,
        remaining_bounces: u64,
        distance: &f64,
        rng: &mut rand::prelude::ThreadRng,
    ) {
        let result: &mut Option<Collision> = &mut Option::None;

        ray.find_collision(
            sphere_vector,
            polygon_vector,
            observer_bodies,
            ray_parameters,
            result,
        );

        let collision: &Option<Collision> = result;

        match collision {
            None => {
                if (ray_parameters.reflect_background)
                    && (remaining_bounces > 0)
                    && (*distance > 0.)
                {
                    self.color_vector.push((
                        ray_parameters.background_color,
                        ray_parameters.background_light_factor,
                    ));
                }
            }
            Some(collision) => {
                let new_distance: f64 = distance + (ray.vector.length * collision.ray_factor);

                self.color_vector.push((
                    apply_light_factor(
                        &collision.object.get_color(),
                        &get_light_factor(
                            &new_distance,
                            &collision.object.get_light_factor(),
                            ray_parameters,
                        ),
                    ),
                    *collision.object.get_light_factor(),
                ));

                if remaining_bounces > 0 {
                    let ray_bounce = ray.get_deviation(
                        *collision.object.get_light_factor(),
                        collision.is_front,
                        &collision.object,
                        ray_parameters,
                        rng,
                    );

                    self.trace_rec(
                        &ray_bounce,
                        ray_parameters,
                        sphere_vector,
                        polygon_vector,
                        observer_bodies,
                        remaining_bounces - 1,
                        &new_distance,
                        rng,
                    );
                }
            }
        }
    }

    fn get_average_color(&self, importance_factor: &f64) -> Color {
        let mut r: f64 = 0.;
        let mut g: f64 = 0.;
        let mut b: f64 = 0.;
        let mut ratio: f64 = 1.;
        let mut total: f64 = 0.;

        self.color_vector.iter().for_each(|(c, f): &(Color, f64)| {
            r += c.r as f64 * f * ratio;
            g += c.g as f64 * f * ratio;
            b += c.b as f64 * f * ratio;
            total += f * ratio;
            ratio *= importance_factor;
        });

        return Color::RGB(
            f64::min(r / total, 255.) as u8,
            f64::min(g / total, 255.) as u8,
            f64::min(b / total, 255.) as u8,
        );
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
