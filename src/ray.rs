use crate::{position::Position, sphere::Sphere};

pub struct Ray {
    pub p1: Position,
    pub p2: Position,
    vector: Position,
    pub l: f64,
    pub x_value: i32,
    pub y_value: i32,
}

impl Ray {
    pub fn new(p1: Position, p2: Position, x_value: i32, y_value: i32) -> Ray {
        return Ray {
            p1,
            p2,
            vector: p2 - p1,
            l: p1.dist(&p2),
            x_value: x_value,
            y_value: y_value,
        };
    }

    pub fn new_turned(
        p: Position,
        d: Position,
        x_value: i32,
        y_value: i32,
        hor_angle: f64,
        ver_angle: f64,
    ) -> Ray {
        let mut r = Ray::new(p, d, x_value, y_value);
        r.turn_hor(hor_angle);
        r.turn_ver(ver_angle);
        return r;
    }

    pub fn turn_hor(&mut self, angle: f64) {
        self.p2.turn_hor_around(angle, &self.p1);
        self.vector = self.p2 - self.p1;
    }

    pub fn turn_ver(&mut self, angle: f64) {
        self.p2.turn_ver_around(angle, &self.p1);
        self.vector = self.p2 - self.p1;
    }

    pub fn factor_from_point(&self, s: &Sphere) -> f64 {
        let x1: f64 = self.p1.x;
        let y1: f64 = self.p1.y;
        let z1: f64 = self.p1.z;
        let x2: f64 = self.p2.x;
        let y2: f64 = self.p2.y;
        let z2: f64 = self.p2.z;
        let x3: f64 = s.pos.x;
        let y3: f64 = s.pos.y;
        let z3: f64 = s.pos.z;
        let a: f64 = (x2 - x1).powf(2.0) + (y2 - y1).powf(2.0) + (z2 - z1).powf(2.0);
        let b: f64 = 2.0 * ((x2 - x1) * (x1 - x3) + (y2 - y1) * (y1 - y3) + (z2 - z1) * (z1 - z3));
        let c: f64 =
            x3.powf(2.0) + y3.powf(2.0) + z3.powf(2.0) + x1.powf(2.0) + y1.powf(2.0) + z1.powf(2.0)
                - 2.0 * (x3 * x1 + y3 * y1 + z3 * z1)
                - s.radius.powf(2.0);

        let d = b.powf(2.0) - 4.0 * a * c;
        if d < 0.0 {
            return f64::NAN;
        } else {
            let ret = (b + f64::sqrt(d)) / (2.0 * a);
            return ret;
        }
    }
}
