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
    vector: &Vec<Sphere>,
    parameters: &Parameters,
    canvas: &mut Canvas<Window>,
) {
    canvas.set_draw_color(parameters.background_color);
    canvas.clear();

    let mut main_factor: f64;
    let mut factor: f64;
    let mut sphere: &Sphere = &Sphere {
        pos: Position {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        v_x: 0.0,
        v_y: 0.0,
        v_z: 0.0,
        radius: 0.0,
        color: Color::RGB(0, 0, 0),
    };
    let colors: &mut Vec<&Color> = &mut Vec::new();
    for r in observer.rays.iter() {
        main_factor = f64::NAN;
        colors.clear();

        for s in vector.iter() {
            let ray_factor = r.factor_distance_from_point(&s);

            if !ray_factor.is_nan() && (main_factor.is_nan() || ray_factor < main_factor) {
                main_factor = ray_factor;
                sphere = s;
            }
        }

        if !main_factor.is_nan() {
            colors.push(&sphere.color);
        }

        if !main_factor.is_nan() {
            factor = main_factor;
            for _ in 0..parameters.ray_bounce_count {
                let bounce_ray = r.get_reflection(sphere, factor);

                factor = f64::NAN;

                for s in vector.iter() {
                    if (s as *const Sphere) != (sphere as *const Sphere) {
                        let ray_factor = bounce_ray.factor_distance_from_point(&s);

                        if !ray_factor.is_nan() && (factor.is_nan() || ray_factor < factor) {
                            factor = ray_factor;
                            sphere = s;
                            colors.push(&s.color);
                        }
                    }
                }

                if factor.is_nan() {
                    break;
                }
            }
        }

        if !colors.is_empty() {
            canvas.set_draw_color(add_fog(
                main_factor,
                r,
                get_average_color(colors, parameters.ray_bounce_color_reflection_factor),
                parameters,
            ));
            // canvas.set_draw_color(sphere.color);
            // canvas.set_draw_color(get_color_with_fog(
            //     sphere,
            //     &r.get_position_from_factor(factor),
            //     parameters,
            // ));
            canvas.draw_point(Point::new(r.x_value, r.y_value)).unwrap();
        }
    }

    canvas.present();
}
