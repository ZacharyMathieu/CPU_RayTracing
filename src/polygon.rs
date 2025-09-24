use sdl2::pixels::Color;

use crate::{object::Object, position::Position, speed::Speed, surface_type::SurfaceType};

#[derive(Clone, Copy)]
pub struct Polygon {
    pub pos: Position,
    pub v1: Position,
    pub v2: Position,
    pub speed: Speed,
    pub color: Color,
    pub light_factor: f64,
    pub type_: SurfaceType,
    pub smoothness: f64,
    pub refractivity_index: f64,
    pub is_visible: bool,
}

impl Polygon {
    pub fn hardcoded_vector() -> Vec<Object> {
        let mut v: Vec<Polygon> = vec![];

        let light_factor: f64 = 1.;
        let size: f64 = 10.;
        // Gold
        v.push(Polygon {
            pos: Position {
                x: -size,
                y: 0.,
                z: 0.,
            },
            v1: Position {
                x: 0.,
                y: size,
                z: 0.,
            },
            v2: Position {
                x: 0.,
                y: 0.,
                z: size,
            },
            speed: Speed {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            color: Color::RGB(255, 200, 0),
            light_factor: light_factor,
            type_: SurfaceType::Reflexive,
            smoothness: 1.,
            refractivity_index: 1.,
            is_visible: true,
        });
        // // Pink
        // v.push(Polygon {
        //     pos: Position {
        //         x: -2.,
        //         y: 0.,
        //         z: 0.,
        //     },
        //     v1: Position {
        //         x: -2.,
        //         y: size * 2.,
        //         z: 0.,
        //     },
        //     v2: Position {
        //         x: -2.,
        //         y: 0.,
        //         z: size * 2.,
        //     },
        //     speed: Speed {
        //         x: 0.,
        //         y: 0.,
        //         z: 0.,
        //     },
        //     color: Color::RGB(255, 125, 255),
        //     light_factor: light_factor,
        //     type_: SurfaceType::Reflexive,
        //     smoothness: 1.,
        //     refractivity_index: 1.,
        //     is_visible: true,
        // });

        return v.iter().map(|p| Object::Polygon(*p)).collect();
    }

    pub fn fill_vector(object_vector: &mut Vec<Object>) {
        object_vector.extend(Polygon::hardcoded_vector());
    }

    pub fn fill_vector_multiple_parameters(object_vector: &mut Vec<Object>) {
        // for sphere_parameters in sphere_parameters_vec {
        Polygon::fill_vector(object_vector);
        // }
    }

    pub fn get_normal(&self) -> Position {
        return self.v1.cross(&self.v2);
    }
}
