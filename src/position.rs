use std::ops::{Add, Sub};

pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position {
    pub fn dist(&self, p: &Position) -> f64 {
        return ((self.x - p.x).powf(2.0) + (self.y - p.y).powf(2.0) + (self.z - p.z).powf(2.0))
            .sqrt();
    }

    pub fn dot_product(&self, p: &Position) -> f64 {
        return (self.x * p.x) + (self.y * p.y) + (self.z * p.z);
    }

    pub fn len_squared(&self) -> f64 {
        return self.x.powf(2.0) + self.y.powf(2.0) + self.y.powf(2.0);
    }
}

impl Add for &Position {
    type Output = Position;

    fn add(self, p: &Position) -> Self::Output {
        return Position {
            x: self.x + p.x,
            y: self.y + p.y,
            z: self.z + p.z,
        };
    }
}

impl Sub for &Position {
    type Output = Position;

    fn sub(self, p: &Position) -> Self::Output {
        return Position {
            x: self.x - p.x,
            y: self.y - p.y,
            z: self.z - p.z,
        };
    }
}
