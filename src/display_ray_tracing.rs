use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::{observer::Observer, ray::Ray, sphere::Sphere};

// fn adjust_point_to_radius(r: &Ray, p: &Position, s: &Sphere, d: f64) -> Position {
//     return *p
//         - (r.d
//             .mul((s.radius.powf(2.0) - d.powf(2.0)).sqrt() / r.l));
// }

fn get_factor_from_point_to_sphere_edge(r: &Ray, s: &Sphere, d: f64) -> f64 {
    return (s.radius.powf(2.0) - d.powf(2.0)).sqrt() / r.l;
}

fn get_adjusted_sphere_color_from_distance(s: &Sphere, d: f64) -> Color {
    let factor = (1.0 - (d / s.radius)).min(1.0);

    return Color::RGB(
        (s.color.r as f64 * factor) as u8,
        (s.color.g as f64 * factor) as u8,
        (s.color.b as f64 * factor) as u8,
    );
}

pub fn display(observer: &Observer, vector: &Vec<Sphere>, canvas: &mut Canvas<Window>) {
    let mut color: Color = Color::RGB(0, 0, 0);

    canvas.set_draw_color(color);
    canvas.clear();

    let mut factor: f64;
    for r in observer.rays.iter() {
        factor = -1.0;

        for sphere in vector.iter() {
            let (sphere_dist, vector_factor) = r.distance_from_point(&sphere.pos);

            if vector_factor >= 0.0 && sphere_dist <= sphere.radius {
                let adjusted_vector_factor =
                    get_factor_from_point_to_sphere_edge(r, sphere, sphere_dist);

                if factor == -1.0 || vector_factor > factor {
                    factor = adjusted_vector_factor;
                    color = get_adjusted_sphere_color_from_distance(sphere, sphere_dist);
                }
            }
        }

        if factor != -1.0 {
            canvas.set_draw_color(color);
            let res = canvas.draw_point(Point::new(r.x_value, r.y_value));
            if res.is_err() {
                println!("{}", res.unwrap_err());
            }
        }
    }

    canvas.present();
}
