use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::{
    observer::Observer, parameters::Parameters, position::Position, ray::Ray, sphere::Sphere,
};

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

pub fn display(
    observer: &Observer,
    sphere_vector: &Vec<Sphere>,
    parameters: &Parameters,
    canvas: &mut Canvas<Window>,
) {
    canvas.set_draw_color(parameters.background_color);
    canvas.clear();

    let color_vector: &mut Vec<&Color> = &mut Vec::new();

    let mut collision: Option<(f64, &Sphere)>;
    let mut bounce_factor: f64;
    let mut bounce_sphere: &Sphere;
    let mut bounce_ray: Ray;

    for ray in observer.rays.iter() {
        color_vector.clear();

        collision = ray.find_collision(sphere_vector, Option::None);

        match collision {
            None => (),
            Some((factor, sphere)) => {
                color_vector.push(&sphere.color);

                bounce_factor = factor;
                bounce_sphere = sphere;

                for _ in 0..parameters.ray_bounce_count {
                    bounce_ray = ray.get_reflection(bounce_factor, bounce_sphere);
                    collision = bounce_ray.find_collision(sphere_vector, Option::Some(sphere));

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

        // if !main_factor.is_nan() {
        //     factor = main_factor;
        //     for _ in 0..parameters.ray_bounce_count {
        //         ray_sphere = sphere;
        //         let bounce_ray = r.get_reflection(sphere, factor);

        //         factor = f64::NAN;

        //         for s in sphere_vector.iter() {
        //             if (s as *const Sphere) != (sphere as *const Sphere) {
        //                 let ray_factor = bounce_ray.factor_distance_from_point(&s);

        //                 if !ray_factor.is_nan() && (factor.is_nan() || ray_factor < factor) {
        //                     factor = ray_factor;
        //                     ray_sphere = s;
        //                 }
        //             }
        //         }

        //         if factor.is_nan() {
        //             break;
        //         } else {
        //             colors.push(&ray_sphere.color);
        //         }
        //     }
        // }

        // if !color_vector.is_empty() {
        // }
    }

    canvas.present();
}
