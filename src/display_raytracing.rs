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

    for z in -parameters.half_h..parameters.half_h {
        for y in -parameters.half_w..parameters.half_w {
            let line = Line {
                p1: &observer.pos,
                p2: &Position {
                    x: observer.direction.x,
                    y: observer.direction.y + y as f64,
                    z: observer.direction.z + z as f64,
                },
            };
            for sphere in vector.iter() {
                let dist = line.dist_point(&sphere.pos);
                if dist > 0.0 && dist <= sphere.radius {
                    canvas.set_draw_color(sphere.color);
                    // println!(
                    //     "x: {xv}\ny: {yv}",
                    //     xv = y + parameters.half_w,
                    //     yv = z + parameters.half_h,
                    // );
                    let res =
                        canvas.draw_point(Point::new(y + parameters.half_w, z + parameters.half_h));
                    if res.is_err() {
                        println!("{}", res.unwrap_err());
                    }
                }
            }
        }
        // println!();
    }

    canvas.present();
}
