use std::ops::{Add, Neg, Sub};

use crate::position::Position;

#[derive(Clone, Copy)]

pub struct Vector {
    pub p0: Position,
    pub p1: Position,
    v: Position,
    pub length: f64,
}

impl Vector {
    pub fn new(p0: Position, p1: Position) -> Vector {
        let l = p0.dist(&p1);
        return Vector {
            p0,
            p1,
            v: p1 - p0,
            length: l,
        };
    }

    pub fn update(&mut self) {
        self.length = self.p0.dist(&self.p1);
        self.v = self.p1 - self.p0;
    }

    pub fn as_position(&self) -> Position {
        return self.v;
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, p: Vector) -> Self::Output {
        return Vector::new(self.p0 + p.p0, self.p1 + p.p1);
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, p: Vector) -> Self::Output {
        return Vector::new(self.p0 - p.p0, self.p1 - p.p1);
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        return Vector {
            p0: -self.p0,
            p1: -self.p1,
            v: -self.v,
            length: self.length,
        };
    }
}
