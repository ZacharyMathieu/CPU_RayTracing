use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::{observer::Observer, parameters::Parameters, sphere::Sphere};

pub fn display(
    observer: &Observer,
    vector: &Vec<Sphere>,
    parameters: &Parameters,
    canvas: &mut Canvas<Window>,
) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    for s in vector.iter() {
        if s.pos.x > observer.pos.x {
            canvas.set_draw_color(s.color);
            let res = canvas.fill_rect(Rect::new(
                (s.pos.y - s.radius - observer.pos.y) as i32,
                (s.pos.z - s.radius - observer.pos.z) as i32,
                (s.radius * 2.0) as u32,
                (s.radius * 2.0) as u32,
            ));
            if res.is_err() {
                println!("{}", res.unwrap_err());
            }
        }
    }

    canvas.present();
}
