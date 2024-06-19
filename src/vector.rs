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
        return Vector {
            p1: -self.p1,
            p2: -self.p2,
            v: -self.v,
            length: self.length,
        };
    }
}
