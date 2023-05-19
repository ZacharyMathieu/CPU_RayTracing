use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::{
    line::Line, observer::Observer, parameters::Parameters, position::Position, sphere::Sphere,
};

pub fn display(
    observer: &Observer,
    vector: &Vec<Sphere>,
    parameters: &Parameters,
    canvas: &mut Canvas<Window>,
) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let mut color: Color = Color::RGB(0, 0, 0);
    let mut factor: f64;
    for z in -(parameters.h_rays as i32 / 2)..=parameters.h_rays as i32 / 2 {
        for y in -(parameters.w_rays as i32 / 2)..=parameters.w_rays as i32 / 2 {
            let line = Line {
                // p1: &Position {
                //     x: observer.pos.x,
                //     y: observer.direction.y + y as f64,
                //     z: observer.direction.z + z as f64,
                // },
                p1: &observer.pos,
                p2: &Position {
                    x: observer.direction.x,
                    y: observer.direction.y + y as f64,
                    z: observer.direction.z + z as f64,
                },
            };

            factor = -1.0;
            for sphere in vector.iter() {
                let (sphere_dist, vector_factor) = line.dist_point(&sphere.pos);
                if vector_factor >= 0.0
                    && sphere_dist <= sphere.radius
                    && (factor == -1.0 || vector_factor > factor)
                {
                    factor = vector_factor;
                    color = sphere.color;
                }
            }

            if factor != -1.0 {
                canvas.set_draw_color(color);
                // println!(
                //     "x: {xv}\ny: {yv}",
                //     xv = y + parameters.half_w,
                //     yv = z + parameters.half_h,
                // );
                let res = canvas.draw_point(Point::new(
                    y + (parameters.h_rays as i32 / 2),
                    z + (parameters.w_rays as i32 / 2),
                ));
                if res.is_err() {
                    println!("{}", res.unwrap_err());
                }
            }
        }
        // println!();
    }

    canvas.present();
}
