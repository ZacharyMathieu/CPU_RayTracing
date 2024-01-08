use sdl2::pixels::Color;

use crate::{parameters::Parameters, ray::Ray, sphere::Sphere};

pub struct RayTrace<'a> {
    pub ray: &'a Ray,
    // pub fog_factor: f64,
    color_vector: Vec<Color>,
    pub color: Color,
}

impl<'a> RayTrace<'a> {
    pub fn new(ray: &'a Ray, parameters: &Parameters) -> Self {
        return RayTrace {
            ray: ray,
            color_vector: Vec::new(),
            color: parameters.background_color,
        };
    }

    pub fn trace(&mut self, sphere_vector: &Vec<Sphere>, parameters: &Parameters) {
        self.trace_rec(
            &mut self.ray.clone(),
            sphere_vector,
            parameters,
            parameters.ray_bounce_count,
        );

        self.set_color(parameters)
    }

    fn set_color(&mut self, parameters: &Parameters) {
        if self.color_vector.len() > 0 {
            self.color = get_average_color(
                &self.color_vector,
                parameters.ray_bounce_color_reflection_factor,
            );
        }
    }

    fn trace_rec(
        &mut self,
        ray: &mut Ray,
        sphere_vector: &Vec<Sphere>,
        parameters: &Parameters,
        remaining_bounces: u32,
    ) {
        if remaining_bounces > 0 {
            let collision: Option<(f64, &Sphere)> = ray.find_collision(sphere_vector);

            match collision {
                None => (),
                Some((factor, sphere)) => {
                    self.color_vector
                        .push(add_fog(sphere.color, ray.length * factor, parameters));

                    if remaining_bounces > 0 {
                        self.trace_rec(
                            &mut ray.get_reflection(factor, sphere),
                            sphere_vector,
                            parameters,
                            remaining_bounces - 1,
                        );
                    }
                }
            };
        }
    }
}

fn add_fog(color: Color, fog_factor: f64, parameters: &Parameters) -> Color {
    let mult = (1.0
        - ((fog_factor * parameters.fog_factor) / parameters.observer_look_vector_distance))
        .max(parameters.min_pixel_factor);

    return Color::RGB(
        (color.r as f64 * mult) as u8,
        (color.g as f64 * mult) as u8,
        (color.b as f64 * mult) as u8,
    );
}

fn get_average_color(
    color_vector: &Vec<Color>,
    importance_factor: f64,
) -> Color {
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
