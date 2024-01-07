use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::{observer::Observer, parameters::Parameters, ray::Ray, sphere::Sphere};

fn add_fog(factor: f64, ray: &Ray, color: Color, parameters: &Parameters) -> Color {
    let mult = (1.0
        - ((factor * ray.l * parameters.fog_factor) / parameters.observer_look_vector_distance))
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

pub fn display_ray(
    sphere_vector: &Vec<Sphere>,
    parameters: &Parameters,
    canvas: &mut Canvas<Window>,
    ray: &Ray,
) {
    let color_vector: &mut Vec<&Color> = &mut Vec::new();

    let mut collision: Option<(f64, &Sphere)> = ray.find_collision(sphere_vector);
    let mut bounce_factor: f64;
    let mut bounce_sphere: &Sphere;
    let mut bounce_ray: Ray;

    match collision {
        None => (),
        Some((factor, sphere)) => {
            color_vector.push(&sphere.color);

            bounce_ray = ray.clone();
            bounce_factor = factor;
            bounce_sphere = sphere;

            for _ in 0..parameters.ray_bounce_count {
                bounce_ray = bounce_ray.get_reflection(bounce_factor, bounce_sphere);
                collision = bounce_ray.find_collision(sphere_vector);

                match collision {
                    None => {
                        color_vector.push(&parameters.background_color);
                        break;
                    }
                    Some((factor, sphere)) => {
                        color_vector.push(&sphere.color);
                        bounce_factor = factor;
                        bounce_sphere = sphere;
                    }
                };
            }

            canvas.set_draw_color(add_fog(
                factor,
                ray,
                get_average_color(color_vector, parameters.ray_bounce_color_reflection_factor),
                parameters,
            ));
            canvas
                .draw_point(Point::new(ray.x_value, ray.y_value))
                .unwrap();
        }
    };
}

pub fn display(
    observer: &Observer,
    sphere_vector: &Vec<Sphere>,
    parameters: &Parameters,
    canvas: &mut Canvas<Window>,
) {
    canvas.set_draw_color(parameters.background_color);
    canvas.clear();

    observer.rays.iter().for_each(|ray| {
        display_ray(sphere_vector, parameters, canvas, ray);
    });

    canvas.present();
}
