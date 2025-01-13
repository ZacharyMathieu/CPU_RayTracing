use rand::{rngs::ThreadRng, Rng};
use sdl2::pixels::Color;

use crate::parameters::{PhysicsParameters, PolygonGenerationMode, PolygonParameters};

#[derive(Clone, Copy)]
pub enum PolygonType {
    Reflexive,
    Refractive,
}

impl PolygonType {
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

pub struct Polygon {
    pub color: Color,
    pub light_factor: f64,
    pub type_: PolygonType,
    pub smoothness: f64,
    pub refractivity_index: f64,
    pub is_visible: bool,
}

impl Polygon {
    pub fn hardcoded_vector() -> Vec<Polygon> {
        let mut v: Vec<Polygon> = vec![];

        let light_factor: f64 = 1.;
        // Blue
        // v.push(Polygon {});

        return v;
    }

    pub fn random_vector(
        polygon_parameters: &PolygonParameters,
        physics_parameters: &PhysicsParameters,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Vec<Polygon> {
        let mut v: Vec<Polygon> = vec![];

        for _ in 0..polygon_parameters.count {
            let new_polygon: Polygon =
                Polygon::random(&polygon_parameters, &physics_parameters, rng);
            v.push(new_polygon);
        }

        return v;
    }

    pub fn random(
        polygon_parameters: &PolygonParameters,
        physics_parameters: &PhysicsParameters,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Polygon {
        let radius_factor: f64 = rng.gen();

        todo!();
        // return Polygon {
        // };
    }

    pub fn fill_polygon_vector(
        polygon_vector: &mut Vec<Polygon>,
        polygon_parameters: &PolygonParameters,
        physics_parameters: &PhysicsParameters,
        rng: &mut ThreadRng,
    ) {
        match polygon_parameters.generation_mode {
            PolygonGenerationMode::Hardcoded => {
                polygon_vector.extend(Polygon::hardcoded_vector());
            }
            PolygonGenerationMode::Random => {
                polygon_vector.extend(Polygon::random_vector(
                    &polygon_parameters,
                    &physics_parameters,
                    rng,
                ));
            }
        }
    }

    pub fn fill_polygon_vector_multiple_parameters(
        polygon_vector: &mut Vec<Polygon>,
        polygon_parameters_vec: &Vec<PolygonParameters>,
        physics_parameters: &PhysicsParameters,
        rng: &mut ThreadRng,
    ) {
        for polygon_parameters in polygon_parameters_vec {
            Polygon::fill_polygon_vector(
                polygon_vector,
                polygon_parameters,
                &physics_parameters,
                rng,
            );
        }
    }
}
