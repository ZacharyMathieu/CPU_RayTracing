use sdl2::pixels::Color;

use crate::{parameters::Parameters, position::Position};

fn float_to_color(f: f64) -> Color {
    return Color::RGB((f * 255.0) as u8, 0, ((1.0 - f) * 255.0) as u8);
}

pub struct Sphere {
    pub pos: Position,
    pub v_x: f64,
    pub v_y: f64,
    pub v_z: f64,
    pub radius: f64,
    pub color: Color,
}

impl Sphere {
    pub fn good_ol_vector(parameters: &Parameters) -> Vec<Sphere> {
        let mut v: Vec<Sphere> = vec![];
        for i in 0..parameters.sphere_count {
            let progress = i as f64 / parameters.sphere_count as f64;
            v.push(Sphere {
                pos: Position {
                    x: 40.0,
                    y: parameters.min_y + (progress * parameters.max_y),
                    z: 0.0,
                },
                // v_x: -0.1 + (progress * 0.2),
                v_x: 0.0,
                // v_y: progress * 0.5,
                v_y: 0.0,
                v_z: 0.0,
                radius: 10.0 + (progress * 10.0),
                color: float_to_color(progress),
            })
        }
        return v;
    }

    // TODO: Optimize... Method too slow because conversion to f64 is bad
    pub fn physics(&mut self, params: &Parameters) {
        self.v_z += params.g;

        self.pos.x += self.v_x;
        self.pos.y += self.v_y;
        self.pos.z += self.v_z;

        if self.pos.x - self.radius < params.min_x {
            self.v_x = self.v_x.abs();
            self.pos.x = params.min_x - (self.pos.x - params.min_x).abs() + self.radius;
        } else if self.pos.x + self.radius > params.max_x {
            self.v_x = -self.v_x.abs();
            self.pos.x = params.max_x - (self.pos.x - params.max_x).abs() - self.radius;
        }

        if self.pos.y - self.radius < params.min_y {
            self.v_y = self.v_y.abs();
            self.pos.y = params.min_y - (self.pos.y - params.min_y).abs() + self.radius;
        } else if self.pos.y + self.radius > params.max_y {
            self.v_y = -self.v_y.abs();
            self.pos.y = params.max_y - (self.pos.y - params.max_y).abs() - self.radius;
        }

        if self.pos.z - self.radius < params.min_z {
            self.v_z = self.v_z.abs();
            self.pos.z = params.min_z - (self.pos.z - params.min_z).abs() + self.radius;
        } else if self.pos.z + self.radius > params.max_z {
            self.v_z = -self.v_z.abs();
            self.pos.z = params.max_z - (self.pos.z - params.max_z).abs() - self.radius;
        }
    }
}
