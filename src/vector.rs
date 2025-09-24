use std::ops::{Add, Neg, Sub};

use crate::position::Position;

#[derive(Clone, Copy)]

pub struct Vector {
    pub origin: Position,
    pub direction: Position,
    pub length: f64,
}

impl Vector {
    pub fn new(origin: Position, direction: Position) -> Vector {
        let length = direction.len();
        return Vector {
            origin,
            direction,
            length,
        };
    }

    pub fn update(&mut self) {
        self.length = self.direction.len();
    }

    pub fn as_position(&self) -> Position {
        return self.direction;
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, p: Vector) -> Self::Output {
        return Vector::new(self.origin + p.origin, self.direction + p.direction);
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, p: Vector) -> Self::Output {
        return Vector::new(self.origin - p.origin, self.direction - p.direction);
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        return Vector {
            origin: self.origin,
            direction: -self.direction,
            length: self.length,
        };
    }
}
