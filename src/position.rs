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

    pub fn len(&self) -> f64 {
        return self.x.powf(2.) + self.y.powf(2.) + self.z.powf(2.);
    }

    fn sin(&self) -> Position {
        return Position {
            x: self.x.sin(),
            y: self.y.sin(),
            z: self.z.sin(),
        };
    }

    fn cos(&self) -> Position {
        return Position {
            x: self.x.cos(),
            y: self.y.cos(),
            z: self.z.cos(),
        };
    }

    pub fn turn(&mut self, angle: Position) {
        let sin = angle.sin();
        let cos = angle.cos();
        let new_x = self.x * cos.x * cos.y
            + self.y * (cos.x * sin.y * sin.z - sin.x * cos.z)
            + self.z * (cos.x * sin.y * cos.z + sin.x * sin.z);
        let new_y = self.x * sin.x * cos.y
            + self.y * (sin.x * sin.y * sin.z + cos.x * cos.z)
            + self.z * (sin.x * sin.y * cos.z - cos.x * sin.z);
        let new_z = self.x * (-sin.y) + self.y * cos.y * sin.z + self.z * cos.y * cos.z;
        self.x = new_x;
        self.y = new_y;
        self.z = new_z;
    }

    pub fn turn_x_around(&mut self, angle: f64, center: &Position) {
        let dy = self.y - center.y;
        let dz = self.z - center.z;
        let sin = angle.sin();
        let cos = angle.cos();

        self.y = (dy * cos) - (dz * sin) + center.y;
        self.z = (dy * sin) + (dz * cos) + center.z;
    }

    pub fn turn_y(&mut self, angle: f64) {
        let sin = angle.sin();
        let cos = angle.cos();
        let new_x = (self.x * cos) + (self.z * sin);
        let new_z = -(self.x * sin) + (self.z * cos);
        self.x = new_x;
        self.z = new_z;
    }

    pub fn turn_y_around(&mut self, angle: f64, center: &Position) {
        let dx = self.x - center.x;
        let dz = self.z - center.z;
        let sin = angle.sin();
        let cos = angle.cos();

        self.x = (dx * cos) - (dz * sin) + center.x;
        self.z = (dx * sin) + (dz * cos) + center.z;
    }

    pub fn turn_z(&mut self, angle: f64) {
        let sin = angle.sin();
        let cos = angle.cos();
        let new_x = (self.x * cos) - (self.y * sin);
        let new_y = (self.x * sin) + (self.y * cos);
        self.x = new_x;
        self.y = new_y;
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

    pub fn dot(&self, p: &Position) -> f64 {
        return (self.x * p.x) + (self.y * p.y) + (self.z * p.z);
    }

    fn length(&self) -> f64 {
        return self.dist(&Position {
            x: 0.,
            y: 0.,
            z: 0.,
        });
    }

    pub fn normalized(&self) -> Position {
        return self.scaled(1. / self.length());
    }

    pub fn angle(&self, v: &Position) -> f64 {
        return f64::acos(
            self.dot(v)
                / (f64::sqrt(
                    (self.x * self.x + self.y * self.y + self.z * self.z)
                        * (v.x * v.x + v.y * v.y + v.z * v.z),
                )),
        );
    }

    pub fn cross(&self, p: &Position) -> Position {
        return Position {
            x: self.y * p.z - self.z * p.y,
            y: self.z * p.x - self.x * p.z,
            z: self.x * p.y - self.y * p.x,
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

impl Mul<Position> for Position {
    type Output = f64;

    fn mul(self, p: Position) -> Self::Output {
        return (self.x * p.x) + (self.y * p.y) + (self.z * p.z);
    }
}

impl Mul<f64> for Position {
    type Output = Position;

    fn mul(self, f: f64) -> Self::Output {
        return Position {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
        };
    }
}
