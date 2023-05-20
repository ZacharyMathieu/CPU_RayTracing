use rand::Rng;
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
    pub fn good_ol_vector(
        parameters: &Parameters,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Vec<Sphere> {
        let mut v: Vec<Sphere> = vec![];
        for i in 0..parameters.sphere_count {
            let progress = i as f64 / (parameters.sphere_count - 1) as f64;
            v.push(Sphere {
                pos: Position {
                    x: parameters.observer_look_vector_distance,
                    y: rng.gen_range(parameters.min_y, parameters.max_y),
                    z: rng.gen_range(parameters.min_z, parameters.max_z),
                },
                v_x: rng.gen_range(-0.1, 0.1),
                v_y: rng.gen_range(-0.1, 0.1),
                v_z: rng.gen_range(-0.1, 0.1),
                radius: rng.gen_range(0.1, 10.0),
                color: float_to_color(progress),
            })
        }
        return v;
    }

    pub fn physics(&mut self, params: &Parameters) {
        self.v_z += params.g;

        self.pos.x += self.v_x;
        self.pos.y += self.v_y;
        self.pos.z += self.v_z;

        if self.pos.x - self.radius < params.min_x {
            self.v_x = self.v_x.abs();
            self.pos.x = params.min_x + self.radius;
        } else if self.pos.x + self.radius > params.max_x {
            self.v_x = -self.v_x.abs();
            self.pos.x = params.max_x - self.radius;
        }

        if self.pos.y - self.radius < params.min_y {
            self.v_y = self.v_y.abs();
            self.pos.y = params.min_y + self.radius;
        } else if self.pos.y + self.radius > params.max_y {
            self.v_y = -self.v_y.abs();
            self.pos.y = params.max_y - self.radius;
        }

        if self.pos.z - self.radius < params.min_z {
            self.v_z = self.v_z.abs();
            self.pos.z = params.min_z + self.radius;
        } else if self.pos.z + self.radius > params.max_z {
            self.v_z = -self.v_z.abs();
            self.pos.z = params.max_z - self.radius;
        }
    }
}
