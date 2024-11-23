use rand::{rngs::ThreadRng, Rng};
use sdl2::pixels::Color;

use crate::{
    parameters::{PhysicsParameters, SphereGenerationMode, SphereParameters},
    position::Position,
    speed::Speed,
    util::{at_ratio, float_to_color, rand_color, rand_range},
};

#[derive(Clone, Copy)]
pub enum SphereType {
    Reflexive,
    Refractive,
}

impl SphereType {
    pub fn to_string(&self) -> &str {
        return match *self {
            Self::Reflexive => "Reflexive",
            Self::Refractive => "Refractive",
        };
    }

    pub fn from_string(string: &str) -> Self {
        return match string {
            "Reflexive" => Self::Reflexive,
            "Refractive" => Self::Refractive,
            _ => Self::Reflexive,
        };
    }
}

#[derive(Clone, Copy)]
pub struct Sphere {
    pub pos: Position,
    pub speed: Speed,
    pub radius: f64,
    pub color: Color,
    pub light_factor: f64,
    pub type_: SphereType,
    pub smoothness: f64,
    pub refractivity_index: f64,
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
            smoothness: 1.,
            refractivity_index: 1.,
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
            smoothness: 0.85,
            refractivity_index: 1.,
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
            smoothness: 0.92,
            refractivity_index: 1.,
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
            color: Color::RGB(255, 0, 255),
            light_factor: 0.,
            type_: SphereType::Refractive,
            smoothness: 1.,
            refractivity_index: 10.,
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
            smoothness: 1.,
            refractivity_index: 0.,
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
            smoothness: 1.,
            refractivity_index: 1.,
            is_visible: true,
        });

        return v;
    }

    pub fn in_line_vector(
        sphere_parameters: &SphereParameters,
        physics_parameters: &PhysicsParameters,
    ) -> Vec<Sphere> {
        let mut v: Vec<Sphere> = vec![];

        for i in 0..sphere_parameters.sphere_count {
            let progress = i as f64 / (sphere_parameters.sphere_count - 1) as f64;
            v.push(Sphere::from_float(
                progress,
                &sphere_parameters,
                &physics_parameters,
            ));
        }

        return v;
    }

    pub fn random_vector(
        sphere_parameters: &SphereParameters,
        physics_parameters: &PhysicsParameters,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Vec<Sphere> {
        let mut v: Vec<Sphere> = vec![];

        for _ in 0..sphere_parameters.sphere_count {
            let new_sphere: Sphere = Sphere::random(&sphere_parameters, &physics_parameters, rng);
            v.push(new_sphere);
        }

        return v;
    }

    pub fn random(
        sphere_parameters: &SphereParameters,
        physics_parameters: &PhysicsParameters,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Sphere {
        let radius_factor: f64 = rng.gen();

        return Sphere {
            pos: Position {
                x: rand_range(rng, physics_parameters.min_x, physics_parameters.max_x),
                y: rand_range(rng, physics_parameters.min_y, physics_parameters.max_y),
                z: rand_range(rng, physics_parameters.min_z, physics_parameters.max_z),
            },
            speed: Speed {
                x: rand_range(rng, physics_parameters.min_vx, physics_parameters.max_vx),
                y: rand_range(rng, physics_parameters.min_vy, physics_parameters.max_vy),
                z: rand_range(rng, physics_parameters.min_vz, physics_parameters.max_vz),
            },
            radius: ((radius_factor
                * (sphere_parameters.max_radius - sphere_parameters.min_radius))
                + sphere_parameters.min_radius),
            color: rand_color(rng),
            light_factor: rand_range(
                rng,
                sphere_parameters.min_light_factor,
                sphere_parameters.max_light_factor,
            ),
            type_: sphere_parameters.sphere_type,
            smoothness: rand_range(
                rng,
                sphere_parameters.min_smoothness,
                sphere_parameters.max_smoothness,
            ),
            refractivity_index: rand_range(
                rng,
                sphere_parameters.min_refractivity_index,
                sphere_parameters.max_refractivity_index,
            ),
            is_visible: true,
        };
    }

    fn from_float(
        f: f64,
        sphere_parameters: &SphereParameters,
        physics_parameters: &PhysicsParameters,
    ) -> Sphere {
        return Sphere {
            pos: Position {
                x: at_ratio(f, physics_parameters.min_x, physics_parameters.max_x),
                y: at_ratio(f, physics_parameters.min_y, physics_parameters.max_y),
                z: at_ratio(f, physics_parameters.min_z, physics_parameters.max_z),
            },
            speed: Speed {
                x: at_ratio(f, physics_parameters.min_vx, physics_parameters.max_vx),
                y: at_ratio(f, physics_parameters.min_vy, physics_parameters.max_vy),
                z: at_ratio(f, physics_parameters.min_vz, physics_parameters.max_vz),
            },
            radius: at_ratio(
                f,
                sphere_parameters.min_radius,
                sphere_parameters.max_radius,
            ),
            color: float_to_color(f),
            light_factor: at_ratio(
                f,
                sphere_parameters.min_light_factor,
                sphere_parameters.max_light_factor,
            ),
            type_: sphere_parameters.sphere_type,
            smoothness: at_ratio(
                f,
                sphere_parameters.min_smoothness,
                sphere_parameters.max_smoothness,
            ),
            refractivity_index: at_ratio(
                f,
                sphere_parameters.min_refractivity_index,
                sphere_parameters.max_refractivity_index,
            ),
            is_visible: true,
        };
    }

    pub fn fill_vector(
        sphere_vector: &mut Vec<Sphere>,
        sphere_parameters: &SphereParameters,
        physics_parameters: &PhysicsParameters,
        rng: &mut ThreadRng,
    ) {
        match sphere_parameters.generation_mode {
            SphereGenerationMode::Hardcoded => {
                sphere_vector.extend(Sphere::hardcoded_vector());
            }
            SphereGenerationMode::InLine => {
                sphere_vector.extend(Sphere::in_line_vector(
                    &sphere_parameters,
                    &physics_parameters,
                ));
            }
            SphereGenerationMode::Random => {
                sphere_vector.extend(Sphere::random_vector(
                    &sphere_parameters,
                    &physics_parameters,
                    rng,
                ));
            }
        }
    }

    pub fn fill_vector_multiple_parameters(
        sphere_vector: &mut Vec<Sphere>,
        sphere_parameters_vec: &Vec<SphereParameters>,
        physics_parameters: &PhysicsParameters,
        rng: &mut ThreadRng,
    ) {
        for sphere_parameters in sphere_parameters_vec {
            Sphere::fill_vector(sphere_vector, sphere_parameters, &physics_parameters, rng);
        }
    }

    pub fn physics(&mut self, physics_parameters: &PhysicsParameters) {
        self.speed.z += physics_parameters.g;

        (self.pos.x, self.speed.x) = self.move_(
            self.pos.x,
            self.speed.x,
            physics_parameters.min_x,
            physics_parameters.max_x,
        );

        (self.pos.y, self.speed.y) = self.move_(
            self.pos.y,
            self.speed.y,
            physics_parameters.min_y,
            physics_parameters.max_y,
        );

        (self.pos.z, self.speed.z) = self.move_(
            self.pos.z,
            self.speed.z,
            physics_parameters.min_z,
            physics_parameters.max_z,
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
