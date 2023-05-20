use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::{observer::Observer, parameters::Parameters, sphere::Sphere};

fn get_adjusted_factor_from_sphere_radius(factor: f64) -> f64 {
    return factor.abs().min(1.0);
}

fn get_adjusted_sphere_color_from_factor(c: &Color, factor: f64, parameters: &Parameters) -> Color {
    fn f(x: f64, fog: f64) -> f64 {
        return 1.0 - (x * fog).min(1.0);
    }

    let factor = ((1.0 - parameters.min_pixel_factor) / f(0.0, parameters.fog_factor))
        * f(factor, parameters.fog_factor)
        + parameters.min_pixel_factor;

    return Color::RGB(
        (c.r as f64 * factor) as u8,
        (c.g as f64 * factor) as u8,
        (c.b as f64 * factor) as u8,
    );
}

pub fn display(
    observer: &Observer,
    vector: &Vec<Sphere>,
    parameters: &Parameters,
    canvas: &mut Canvas<Window>,
) {
    let mut color: &Color = &Color::RGB(0, 0, 0);

    canvas.set_draw_color(*color);
    canvas.clear();

    let mut factor: f64;
    for r in observer.rays.iter() {
        factor = f64::NAN;

        for sphere in vector.iter() {
            let ray_factor = r.factor_from_point(&sphere);

            if !ray_factor.is_nan() && (factor.is_nan() || ray_factor > factor) {
                factor = ray_factor;
                color = &sphere.color;
            }
        }

        if !factor.is_nan() {
            canvas.set_draw_color(get_adjusted_sphere_color_from_factor(
                color,
                get_adjusted_factor_from_sphere_radius(factor),
                parameters,
            ));
            canvas.draw_point(Point::new(r.x_value, r.y_value)).unwrap();
        }
    }

    canvas.present();
}
