use sdl2::pixels::Color;

use crate::{polygon::Polygon, sphere::Sphere};

pub enum Object<'a> {
    Sphere(&'a Sphere),
    Polygon(&'a Polygon),
}

impl<'a> Object<'a> {
    pub fn get_color(&self) -> &Color {
        return match self {
            Object::Sphere(sphere) => &sphere.color,
            Object::Polygon(polygon) => &polygon.color,
        };
    }

    pub fn get_light_factor(&self) -> &f64 {
        return match self {
            Object::Sphere(sphere) => &sphere.light_factor,
            Object::Polygon(polygon) => &polygon.light_factor,
        };
    }

    pub fn get_smoothness(&self) -> &f64 {
        return match self {
            Object::Sphere(sphere) => &sphere.smoothness,
            Object::Polygon(polygon) => &polygon.smoothness,
        };
    }
}
