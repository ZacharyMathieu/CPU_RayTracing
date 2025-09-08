use sdl2::pixels::Color;

use crate::{polygon::Polygon, sphere::Sphere, surface_type::SurfaceType};

pub enum Object {
    Sphere(Sphere),
    Polygon(Polygon),
}

impl Object {
    pub fn color(&self) -> Color {
        match self {
            Object::Sphere(s) => return s.color,
            Object::Polygon(p) => return p.color,
        }
    }

    pub fn light_factor(&self) -> f64 {
        match self {
            Object::Sphere(s) => return s.light_factor,
            Object::Polygon(p) => return p.light_factor,
        }
    }

    pub fn type_(&self) -> SurfaceType {
        match self {
            Object::Sphere(s) => return s.type_,
            Object::Polygon(p) => return p.type_,
        }
    }

    pub fn smoothness(&self) -> f64 {
        match self {
            Object::Sphere(s) => return s.smoothness,
            Object::Polygon(p) => return p.smoothness,
        }
    }

    pub fn refractivity_index(&self) -> f64 {
        match self {
            Object::Sphere(s) => return s.refractivity_index,
            Object::Polygon(p) => return p.refractivity_index,
        }
    }

    pub fn is_visible(&self) -> bool {
        match self {
            Object::Sphere(s) => return s.is_visible,
            Object::Polygon(p) => return p.is_visible,
        }
    }
}
