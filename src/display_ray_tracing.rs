use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::{
    observer::Observer, parameters::Parameters, position::Position, ray::Ray, sphere::Sphere,
};

fn get_color_with_fog(s: &Sphere, p: &Position, parameters: &Parameters) -> Color {
    fn f(x: f64, parameters: &Parameters) -> f64 {
        return (x * parameters.fog_factor)
            .max(parameters.min_pixel_factor)
            .min(1.0);
    }

    let mult = f(
        1.0 - ((p.z - (s.pos.z - s.radius)) / (s.radius * 2.0)),
        parameters,
    );

    return Color::RGB(
        (s.color.r as f64 * mult) as u8,
        (s.color.g as f64 * mult) as u8,
        (s.color.b as f64 * mult) as u8,
    );
}

pub fn get_average_color(color_vector: &Vec<&Color>) -> Color {
    let mut r: u128 = 0;
    let mut g: u128 = 0;
    let mut b: u128 = 0;

    for c in color_vector {
        r += c.r as u128;
        g += c.g as u128;
        b += c.b as u128;
    }

    return Color::RGB(
        (r / color_vector.len() as u128) as u8,
        (g / color_vector.len() as u128) as u8,
        (b / color_vector.len() as u128) as u8,
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
    // let colors: &mut Vec<&Color> = &mut Vec::new();
    for r in observer.rays.iter() {
        factor = f64::NAN;

        for s in vector.iter() {
            let ray_factor = r.factor_distance_from_point(&s);

            if !ray_factor.is_nan() && (factor.is_nan() || ray_factor < factor) {
                factor = ray_factor;
                sphere = s;
                // colors.push(&s.color);
            }
        }

        for _ in 0..parameters.ray_bounce_count {
            if factor.is_nan() {
                break;
            }

            let bounce_ray = Ray::new(
                sphere.pos,
                r.get_position_from_factor(factor),
                r.x_value,
                r.y_value,
            );

            for s in vector.iter() {
                // if (s as *const Sphere) != (sphere as *const Sphere) {
                let ray_factor = bounce_ray.factor_distance_from_point(&s);

                if !ray_factor.is_nan() && (factor.is_nan() || ray_factor > factor) {
                    factor = ray_factor;
                    sphere = s;
                    // colors.push(&s.color);
                }
                // }
            }
        }

        if !factor.is_nan() {
            // canvas.set_draw_color(get_average_color(colors));
            canvas.set_draw_color(sphere.color);
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
