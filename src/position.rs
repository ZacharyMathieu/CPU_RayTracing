use std::ops::{Add, Sub};

#[derive(Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub name: String,
}

impl Position {
    pub fn dist_squared(&self, p: &Position) -> f64 {
        return (self.x - p.x).powf(2.0) + (self.y - p.y).powf(2.0) + (self.z - p.z).powf(2.0);
    }

    pub fn dist(&self, p: &Position) -> f64 {
        return self.dist_squared(p).sqrt();
    }

    pub fn dot_product(&self, p: &Position) -> f64 {
        return (self.x * p.x) + (self.y * p.y) + (self.z * p.z);
    }

    pub fn turn_hor_around(&mut self, angle: f64, center: &Position) {
        let dx = self.x - center.x;
        let dy = self.y - center.y;
        let sin = angle.sin();
        let cos = angle.cos();

        self.x = (dx * cos) - (dy * sin) + center.x;
        self.y = (dx * sin) + (dy * cos) + center.y;
    }

    pub fn turn_ver_around(&mut self, angle: f64, center: &Position) {
        let dx = self.x - center.x;
        let dz = self.z - center.z;
        let sin = angle.sin();
        let cos = angle.cos();

        self.x = (dx * cos) - (dz * sin) + center.x;
        self.z = (dx * sin) + (dz * cos) + center.z;
    }

    pub fn scaled(&self, factor: f64) -> Position {
        return Position {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
            name: self.name.clone() + "_copy",
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
            name: format!("({name_1} + {name_2})", name_1 = self.name, name_2 = p.name),
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
            name: format!("({name_1} - {name_2})", name_1 = self.name, name_2 = p.name),
        };
    }
}
