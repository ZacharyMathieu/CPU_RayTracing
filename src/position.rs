use std::ops::{Add, Mul, Neg, Sub};

#[derive(Clone, Copy)]

pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position {
    pub fn dist_squared(&self, p: &Position) -> f64 {
        return (self.x - p.x).powf(2.) + (self.y - p.y).powf(2.) + (self.z - p.z).powf(2.);
    }

    pub fn dist(&self, p: &Position) -> f64 {
        return self.dist_squared(p).sqrt();
    }

    pub fn dot_product(&self, p: &Position) -> f64 {
        return (self.x * p.x) + (self.y * p.y) + (self.z * p.z);
    }

    pub fn turn_x_around(&mut self, angle: f64, center: &Position) {
        let dy = self.y - center.y;
        let dz = self.z - center.z;
        let sin = angle.sin();
        let cos = angle.cos();

        self.y = (dy * cos) - (dz * sin) + center.y;
        self.z = (dy * sin) + (dz * cos) + center.z;
    }

    pub fn turn_y_around(&mut self, angle: f64, center: &Position) {
        let dx = self.x - center.x;
        let dz = self.z - center.z;
        let sin = angle.sin();
        let cos = angle.cos();

        self.x = (dx * cos) - (dz * sin) + center.x;
        self.z = (dx * sin) + (dz * cos) + center.z;
    }

    pub fn turn_z_around(&mut self, angle: f64, center: &Position) {
        let dx = self.x - center.x;
        let dy = self.y - center.y;
        let sin = angle.sin();
        let cos = angle.cos();

        self.x = (dx * cos) - (dy * sin) + center.x;
        self.y = (dx * sin) + (dy * cos) + center.y;
    }

    pub fn scaled(&self, factor: f64) -> Position {
        return Position {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        };
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, p: Position) -> Self::Output {
        return Position {
            x: self.x + p.x,
            y: self.y + p.y,
            z: self.z + p.z,
        };
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, p: Position) -> Self::Output {
        return Position {
            x: self.x - p.x,
            y: self.y - p.y,
            z: self.z - p.z,
        };
    }
}

impl Neg for Position {
    type Output = Position;

    fn neg(self) -> Self::Output {
        return Position {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        };
    }
}

// impl Mul for Position {
//     type Output = f64;

//     fn mul(self, p: Position) -> Self::Output {
//         return self.x * p.x + self.y * p.y + self.z * p.z;
//     }
// }
