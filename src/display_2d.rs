use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::{sphere::Sphere, parameters::Parameters};

pub fn display(observer: &Sphere, vector: &Vec<Sphere>, parameters: &Parameters, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    for s in vector.iter() {
        canvas.set_draw_color(s.color);
        let res = canvas.fill_rect(Rect::new(
            (s.p.x - s.radius - observer.p.x) as i32,
            (s.p.y - s.radius - observer.p.y) as i32,
            (s.radius * 2.0) as u32,
            (s.radius * 2.0) as u32,
        ));
        if res.is_err() {
            println!("{}", res.unwrap_err());
        }
    }

    canvas.present();
}
