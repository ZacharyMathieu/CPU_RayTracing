use sdl2::pixels::Color;

use crate::{parameters::Parameters, position::Position};

fn float_to_color(f: f64) -> Color {
    return Color::RGB((f * 255.0) as u8, 0, ((1.0 - f) * 255.0) as u8);
}

pub struct Sphere {
    pub p: Position,
    pub v_x: f64,
    pub v_y: f64,
    pub radius: f64,
    pub color: Color,
}

impl Sphere {
    pub fn good_ol_vector(size: u32) -> Vec<Sphere> {
        let mut v: Vec<Sphere> = vec![];
        for i in 0..size {
            let progress = i as f64 / size as f64;
            v.push(Sphere {
                p: Position {
                    x: 25.0 + (i as f64 * 50.0),
                    y: 25.0 + (i as f64 * 50.0),
                    z: 25.0 + (i as f64 * 50.0),
                },
                v_x: progress * 0.5,
                v_y: 0.0,
                radius: progress * 15.0,
                // radius: 0.0,
                color: float_to_color(progress),
            })
        }
        return v;
    }

    // TODO: Optimize... Method too slow because conversion to f64 is bad
    pub fn physics(&mut self, params: &Parameters) {
        let mul = 1.0;
        let div = 1.0;

        self.v_y += params.g;

        self.p.x += self.v_x;
        self.p.y += self.v_y;

        let w = params.w as f64;
        if self.p.x - self.radius < 0.0 {
            self.v_x = self.v_x.abs();
            self.p.x = self.p.x.abs();
        } else if self.p.x + self.radius > w {
            self.v_x = -self.v_x.abs();
            self.p.x = w - (self.p.x - w).abs();
        }

        let h = params.h as f64;
        if self.p.y - self.radius < 0.0 {
            self.v_y = self.v_y.abs() / div;
            self.p.y = self.p.y.abs();
        } else if self.p.y + self.radius > h {
            self.v_y = -self.v_y.abs() * mul;
            self.p.y = h - (self.p.y - h).abs();
        }
    }
}
