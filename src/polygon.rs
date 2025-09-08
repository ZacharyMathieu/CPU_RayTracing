use sdl2::pixels::Color;

use crate::{object::Object, position::Position, speed::Speed, surface_type::SurfaceType};

#[derive(Clone, Copy)]
pub struct Polygon {
    pub pos: Position,
    pub v1: Position,
    pub v2: Position,
    pub speed: Speed,
    pub radius: f64,
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
        let size: f64 = 1.;
        // Blue
        v.push(Polygon {
            pos: Position {
                x: 10.,
                y: 0.,
                z: 0.,
            },
            v1: Position {
                x: 10.,
                y: 0.,
                z: 0.,
            },
            v2: Position {
                x: 10.,
                y: 0.,
                z: 0.,
            },
            speed: Speed {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            radius: size,
            color: Color::RGB(0, 0, 255),
            light_factor: light_factor,
            type_: SurfaceType::Reflexive,
            smoothness: 1.,
            refractivity_index: 1.,
            is_visible: true,
        });

        return v.iter().map(|p| Object::Polygon(*p)).collect();
    }
}
