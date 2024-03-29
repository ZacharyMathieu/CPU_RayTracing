use rand::Rng;
use sdl2::pixels::Color;

use crate::{
    parameters::Parameters,
    position::Position,
    util::{rand_color, rand_range},
};

pub struct Sphere {
    pub pos: Position,
    pub v_x: f64,
    pub v_y: f64,
    pub v_z: f64,
    pub radius: f64,
    pub color: Color,
    pub light_factor: f64,
    pub reflexivity_factor: f64,
}

impl Sphere {
    pub fn good_ol_vector(
        parameters: &Parameters,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Vec<Sphere> {
        let mut v: Vec<Sphere> = vec![];

        // let light_factor: f64 = 0.1;
        // // Blue
        // v.push(Sphere {
        //     pos: Position {
        //         x: 10.0,
        //         y: 0.0,
        //         z: 0.0,
        //     },
        //     v_x: 0.0,
        //     v_y: 0.0,
        //     v_z: 0.0,
        //     radius: 6.0,
        //     color: Color::RGB(0, 0, 255),
        //     light_factor: light_factor,
        // });
        // // Red
        // v.push(Sphere {
        //     pos: Position {
        //         x: 10.0,
        //         y: -5.0,
        //         z: 0.0,
        //     },
        //     v_x: 0.0,
        //     v_y: 0.0,
        //     v_z: 0.0,
        //     radius: 2.0,
        //     color: Color::RGB(255, 0, 0),
        //     light_factor: light_factor,
        // });
        // // Green
        // v.push(Sphere {
        //     pos: Position {
        //         x: 10.0,
        //         y: -4.0,
        //         z: 2.5,
        //     },
        //     v_x: 0.0,
        //     v_y: 0.0,
        //     v_z: 0.0,
        //     radius: 3.0,
        //     color: Color::RGB(0, 255, 0),
        //     light_factor: light_factor,
        // });
        // // Yellow
        // v.push(Sphere {
        //     pos: Position {
        //         x: 2.0,
        //         y: 0.0,
        //         z: 0.0,
        //     },
        //     v_x: 0.0,
        //     v_y: 0.0,
        //     v_z: 0.0,
        //     radius: 1.5,
        //     color: Color::RGB(255, 255, 0),
        //     light_factor: light_factor,
        // });
        // // Turquoise
        // v.push(Sphere {
        //     pos: Position {
        //         x: 4.0,
        //         y: 0.0,
        //         z: 0.0,
        //     },
        //     v_x: 0.0,
        //     v_y: 0.0,
        //     v_z: 0.0,
        //     radius: 0.25,
        //     color: Color::RGB(0, 255, 255),
        //     light_factor: light_factor,
        // });
        // // White
        // v.push(Sphere {
        //     pos: Position {
        //         x: -20.0,
        //         y: 20.0,
        //         z: -20.0,
        //     },
        //     v_x: 0.0,
        //     v_y: 0.0,
        //     v_z: 0.0,
        //     radius: 10.,
        //     color: Color::RGB(255, 255, 255),
        //     light_factor: 100.,
        // });

        // for i in 0..parameters.sphere_parameters.sphere_count {
        //     let progress = i as f64 / (parameters.sphere_parameters.sphere_count - 1) as f64;
        //     v.push(Sphere::generate_random(
        //         parameters,
        //         rng,
        //         float_to_color(progress),
        //     ));
        // }

        for _ in 0..parameters.sphere_parameters.sphere_count {
            let color: Color = rand_color(rng);
            v.push(Sphere::generate_random(parameters, rng, color));
        }

        return v;
    }

    pub fn generate_random(
        parameters: &Parameters,
        rng: &mut rand::prelude::ThreadRng,
        color: Color,
    ) -> Sphere {
        let radius_factor: f64 = rng.gen();

        return Sphere {
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
            v_x: rand_range(
                rng,
                parameters.physics_parameters.min_vx,
                parameters.physics_parameters.max_vx,
            ),
            v_y: rand_range(
                rng,
                parameters.physics_parameters.min_vy,
                parameters.physics_parameters.max_vy,
            ),
            v_z: rand_range(
                rng,
                parameters.physics_parameters.min_vz,
                parameters.physics_parameters.max_vz,
            ),
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
            reflexivity_factor: rand_range(
                rng,
                parameters.sphere_parameters.min_reflexivity_factor,
                parameters.sphere_parameters.max_reflexivity_factor,
            ),
        };
    }

    pub fn physics(&mut self, params: &Parameters) {
        self.v_z += params.physics_parameters.g;

        self.pos.x += self.v_x;
        self.pos.y += self.v_y;
        self.pos.z += self.v_z;

        // TODO remove the duplicated code
        if self.pos.x - self.radius < params.physics_parameters.min_x {
            self.v_x = self.v_x.abs();
            self.pos.x = params.physics_parameters.min_x + self.radius;
        } else if self.pos.x + self.radius > params.physics_parameters.max_x {
            self.v_x = -self.v_x.abs();
            self.pos.x = params.physics_parameters.max_x - self.radius;
        }

        if self.pos.y - self.radius < params.physics_parameters.min_y {
            self.v_y = self.v_y.abs();
            self.pos.y = params.physics_parameters.min_y + self.radius;
        } else if self.pos.y + self.radius > params.physics_parameters.max_y {
            self.v_y = -self.v_y.abs();
            self.pos.y = params.physics_parameters.max_y - self.radius;
        }

        if self.pos.z - self.radius < params.physics_parameters.min_z {
            self.v_z = self.v_z.abs();
            self.pos.z = params.physics_parameters.min_z + self.radius;
        } else if self.pos.z + self.radius > params.physics_parameters.max_z {
            self.v_z = -self.v_z.abs();
            self.pos.z = params.physics_parameters.max_z - self.radius;
        }
    }
}
