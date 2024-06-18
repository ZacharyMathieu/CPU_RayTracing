use std::ops::{Add, Neg, Sub};

use crate::position::Position;

#[derive(Clone, Copy)]

pub struct Vector {
    pub p1: Position,
    pub p2: Position,
    v: Position,
    pub length: f64,
}

impl Vector {
    pub fn new(p1: Position, p2: Position) -> Vector {
        let l = p1.dist(&p2);
        return Vector {
            p1,
            p2,
            v: p2 - p1,
            length: l,
        };
    }

    pub fn update(&mut self) {
        self.length = self.p1.dist(&self.p2);
        self.v = self.p2 - self.p1;
    }

    pub fn as_position(&self) -> Position {
        return self.v;
    }

    pub fn dot(&self, v: &Vector) -> f64 {
        return self.v.x * v.v.x + self.v.y * v.v.y + self.v.z * v.v.z;
    }

    pub fn angle(&self, v: &Vector) -> f64 {
        return f64::acos(self.dot(v) / (self.length * v.length));
    }

    pub fn from_angle(&self, a: f64, v: &Vector) -> Vector {
        // cos(a) * (self.length * v.length) = self.dot(v)
        let base: f64 = f64::cos(a) * (self.length * v.length);
        let x: f64 = base - (self.v.y * v.v.y + self.v.z * v.v.z);
        let y: f64 = base - (self.v.x * v.v.x + self.v.z * v.v.z);
        let z: f64 = base - (self.v.x * v.v.x + self.v.y * v.v.y);

        return Vector::new(
            Position {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            Position { x, y, z },
        );
    }

    pub fn scaled(&self, factor: f64) -> Vector {
        return Vector::new(self.p1.scaled(factor), self.p2.scaled(factor));
    }

    pub fn set_origin(&mut self, pos: &Position) {
        self.p2 = self.p2 - self.p1 + *pos;
        self.p1 = *pos;
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, p: Vector) -> Self::Output {
        return Vector::new(self.p1 + p.p1, self.p2 + p.p2);
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, p: Vector) -> Self::Output {
        return Vector::new(self.p1 - p.p1, self.p2 - p.p2);
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        return Vector::new(-self.p1, -self.p2);
    }
}