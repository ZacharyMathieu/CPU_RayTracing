use rand::Rng;
use sdl2::pixels::Color;

use crate::{
    parameters::Parameters,
    position::Position,
    speed::Speed,
    util::{float_to_color, rand_color, rand_range},
};

#[derive(Clone, Copy)]

pub enum SphereType {
    Reflexive,
    Refractive,
}

#[derive(Clone, Copy)]

pub struct Sphere {
    pub pos: Position,
    pub speed: Speed,
    pub radius: f64,
    pub color: Color,
    pub light_factor: f64,
    pub type_: SphereType,
    pub reflexivity_factor: f64,
    pub refractivity_factor: f64,
    pub is_visible: bool,
}

impl Sphere {
    pub fn hardcoded_vector() -> Vec<Sphere> {
        let mut v: Vec<Sphere> = vec![];

        let light_factor: f64 = 1.;
        // Blue
        v.push(Sphere {
            pos: Position {
                x: 10.,
                y: 0.,
                z: 0.,
            },
            speed: Speed {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            radius: 6.,
            color: Color::RGB(0, 0, 255),
            light_factor: light_factor,
            type_: SphereType::Reflexive,
            reflexivity_factor: 0.,
            refractivity_factor: 1.,
            is_visible: true,
        });
        // Red
        v.push(Sphere {
            pos: Position {
                x: 10.,
                y: -5.,
                z: 0.,
            },
            speed: Speed {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            radius: 2.,
            color: Color::RGB(255, 0, 0),
            light_factor: light_factor,
            type_: SphereType::Reflexive,
            reflexivity_factor: 0.15,
            refractivity_factor: 1.,
            is_visible: true,
        });
        // Green
        v.push(Sphere {
            pos: Position {
                x: 10.,
                y: -4.,
                z: 2.5,
            },
            speed: Speed {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            radius: 3.,
            color: Color::RGB(0, 255, 0),
            light_factor: light_factor,
            type_: SphereType::Reflexive,
            reflexivity_factor: 0.02,
            refractivity_factor: 1.,
            is_visible: true,
        });
        // White Refractive
        v.push(Sphere {
            pos: Position {
                x: 2.,
                y: 0.,
                z: 0.,
            },
            speed: Speed {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            radius: 1.5,
            color: Color::RGB(255, 255, 255),
            light_factor: 0.,
            type_: SphereType::Refractive,
            reflexivity_factor: 0.,
            refractivity_factor: 1.2,
            is_visible: true,
        });
        // Turquoise
        v.push(Sphere {
            pos: Position {
                x: 4.,
                y: 0.,
                z: 0.,
            },
            speed: Speed {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            radius: 0.25,
            color: Color::RGB(0, 255, 255),
            light_factor: light_factor,
            type_: SphereType::Reflexive,
            reflexivity_factor: 0.,
            refractivity_factor: 0.,
            is_visible: true,
        });
        // White
        v.push(Sphere {
            pos: Position {
                x: -20.,
                y: 20.,
                z: -20.,
            },
            speed: Speed {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            radius: 10.,
            color: Color::RGB(255, 255, 255),
            light_factor: 2.,
            type_: SphereType::Refractive,
            reflexivity_factor: 0.,
            refractivity_factor: 1.,
            is_visible: true,
        });

        return v;
    }

    pub fn in_line_vector(
        parameters: &Parameters,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Vec<Sphere> {
        let mut v: Vec<Sphere> = vec![];

        for i in 0..parameters.sphere_parameters.sphere_count {
            let progress = i as f64 / (parameters.sphere_parameters.sphere_count - 1) as f64;
            v.push(Sphere::generate_random(
                parameters,
                rng,
                float_to_color(progress),
            ));
        }

        return v;
    }
    pub fn random_vector(
        parameters: &Parameters,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Vec<Sphere> {
        let mut v: Vec<Sphere> = vec![];

        for _ in 0..parameters.sphere_parameters.sphere_count {
            let color: Color = rand_color(rng);
            let new_sphere: Sphere = Sphere::generate_random(parameters, rng, color);
            v.push(new_sphere);
        }

        return v;
    }

    pub fn generate_random(
        parameters: &Parameters,
        rng: &mut rand::prelude::ThreadRng,
        color: Color,
    ) -> Sphere {
        let radius_factor: f64 = rng.gen();

        let new_sphere: Sphere = Sphere {
            pos: Position {
                x: rand_range(
                    rng,
                    parameters.physics_parameters.min_x,
                    parameters.physics_parameters.max_x,
                ),
                y: rand_range(
                    rng,
                    parameters.physics_parameters.min_y,
                    parameters.physics_parameters.max_y,
                ),
                z: rand_range(
                    rng,
                    parameters.physics_parameters.min_z,
                    parameters.physics_parameters.max_z,
                ),
            },
            speed: Speed {
                x: rand_range(
                    rng,
                    parameters.physics_parameters.min_vx,
                    parameters.physics_parameters.max_vx,
                ),
                y: rand_range(
                    rng,
                    parameters.physics_parameters.min_vy,
                    parameters.physics_parameters.max_vy,
                ),
                z: rand_range(
                    rng,
                    parameters.physics_parameters.min_vz,
                    parameters.physics_parameters.max_vz,
                ),
            },
            radius: ((radius_factor
                * (parameters.sphere_parameters.max_radius
                    - parameters.sphere_parameters.min_radius))
                + parameters.sphere_parameters.min_radius),
            color: color,
            light_factor: rand_range(
                rng,
                parameters.sphere_parameters.min_light_factor,
                parameters.sphere_parameters.max_light_factor,
            ),
            type_: parameters.sphere_parameters.sphere_types[rand_range(
                rng,
                0 as usize,
                parameters.sphere_parameters.sphere_types.len(),
            ) as usize],
            reflexivity_factor: rand_range(
                rng,
                parameters.sphere_parameters.min_reflexivity_factor,
                parameters.sphere_parameters.max_reflexivity_factor,
            ),
            refractivity_factor: rand_range(
                rng,
                parameters.sphere_parameters.min_refractivity_factor,
                parameters.sphere_parameters.max_refractivity_factor,
            ),
            is_visible: true,
        };
        return new_sphere;
    }

    pub fn physics(&mut self, params: &Parameters) {
        self.speed.z += params.physics_parameters.g;

        (self.pos.x, self.speed.x) = self.move_(
            self.pos.x,
            self.speed.x,
            params.physics_parameters.min_x,
            params.physics_parameters.max_x,
        );

        (self.pos.y, self.speed.y) = self.move_(
            self.pos.y,
            self.speed.y,
            params.physics_parameters.min_y,
            params.physics_parameters.max_y,
        );

        (self.pos.z, self.speed.z) = self.move_(
            self.pos.z,
            self.speed.z,
            params.physics_parameters.min_z,
            params.physics_parameters.max_z,
        );
    }

    fn move_(&self, pos: f64, speed: f64, min: f64, max: f64) -> (f64, f64) {
        let mut new_pos: f64 = pos;
        let mut new_speed: f64 = speed;

        new_pos += new_speed;
        if new_pos - self.radius < min {
            new_speed = new_speed.abs();
            new_pos = min + self.radius;
        } else if new_pos + self.radius > max {
            new_speed = -new_speed.abs();
            new_pos = max - self.radius;
        }

        return (new_pos, new_speed);
    }
}
