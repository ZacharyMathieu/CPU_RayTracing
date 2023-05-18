use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::{line::Line, parameters::Parameters, position::Position, sphere::Sphere};

pub fn display(
    observer: &Sphere,
    vector: &Vec<Sphere>,
    parameters: &Parameters,
    canvas: &mut Canvas<Window>,
) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    for z in 0..parameters.h {
        for y in 0..parameters.w {
            let line = Line {
                p1: &observer.p,
                p2: &Position {
                    x: observer.p.x,
                    y: observer.p.y + (y - (parameters.w / 2)) as f64,
                    z: observer.p.z + (z - (parameters.h / 2)) as f64,
                },
            };
            for sphere in vector.iter() {
                let dist = line.dist_point(&sphere.p);
                if dist <= sphere.radius {
                    canvas.set_draw_color(sphere.color);
                    let res = canvas.draw_point(Point::new(y as i32, z as i32));
                    if res.is_err() {
                        println!("{}", res.unwrap_err());
                    }
                }
            }
        }
    }

    // for s in vector.iter() {
    //     canvas.set_draw_color(s.color);
    //     let res = canvas.draw_point(Point::new((s.x - s.radius) as i32, (s.y - s.radius) as i32));
    //     if res.is_err() {
    //         println!("{}", res.unwrap_err());
    //     }
    // }

    canvas.present();
}
