use crate::{position::Position, sphere::Sphere};

fn squared(f: f64) -> f64 {
    return f * f;
}

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

    pub fn get_position_from_factor(&self, factor: f64) -> Position {
        return self.p1 + (self.p2.scaled(factor));
    }

    pub fn factor_distance_from_point(&self, s: &Sphere) -> f64 {
        // These are the parts of a quadratic equation given by substituting
        // the values of the line (ray) into the equation for the given sphere
        let a: f64 = squared(self.p2.x - self.p1.x)
            + squared(self.p2.y - self.p1.y)
            + squared(self.p2.z - self.p1.z);
        let b: f64 = 2.0
            * ((self.p2.x - self.p1.x) * (self.p1.x - s.pos.x)
                + (self.p2.y - self.p1.y) * (self.p1.y - s.pos.y)
                + (self.p2.z - self.p1.z) * (self.p1.z - s.pos.z));
        let c: f64 = squared(s.pos.x)
            + squared(s.pos.y)
            + squared(s.pos.z)
            + squared(self.p1.x)
            + squared(self.p1.y)
            + squared(self.p1.z)
            - 2.0 * (s.pos.x * self.p1.x + s.pos.y * self.p1.y + s.pos.z * self.p1.z)
            - squared(s.radius);

        // Guard clause :D
        let d = squared(b) - 4.0 * a * c;
        if d < 0.0 {
            return f64::NAN;
        }

        let ret = (b - f64::sqrt(d)) / (2.0 * a);

        // Remove the points behind the observer
        if ret > 0.0 {
            return f64::NAN;
        }

        return ret;
    }
}
