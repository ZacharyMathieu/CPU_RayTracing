use crate::position::Position;

pub struct Ray {
    pub p: Position,
    pub d: Position,
    vector: Position,
    pub l: f64,
    pub x_value: i32,
    pub y_value: i32,
}

impl Ray {
    pub fn new(p: Position, d: Position, x_value: i32, y_value: i32) -> Ray {
        return Ray {
            p: p,
            d: d,
            vector: d - p,
            l: p.dist(&d),
            x_value: x_value,
            y_value: y_value,
        };
    }

    // pub fn get_p(&self) -> &Position {
    //     return &self.p;
    // }

    // pub fn get_d(&self) -> &Position {
    //     return &self.d;
    // }

    pub fn turn_hor(&mut self, angle: f64) {
        self.d.turn_hor_around(angle, &self.p);
        self.vector = self.d - self.p;
    }

    pub fn turn_ver(&mut self, angle: f64) {
        self.d.turn_ver_around(angle, &self.p);
        self.vector = self.d - self.p;
    }

    pub fn generate_turned(&self, hor_angle: f64, ver_angle: f64, new_x: i32, new_y: i32) -> Ray {
        let mut ray = Ray::new(self.p, self.d, new_x, new_y);

        ray.turn_hor(hor_angle);
        ray.turn_ver(ver_angle);

        return ray;
    }

    pub fn distance_from_point(&self, p: &Position) -> (f64, f64) {
        // let factor =
        //     -(((self.p - *p).dot_product(&(self.d - self.p))) / (self.d - self.p).len_squared());
        // let closest_point = Position {
        //     x: self.p.x + ((self.d.x - self.p.x) * factor),
        //     y: self.p.y + ((self.d.y - self.p.y) * factor),
        //     z: self.p.z + ((self.d.z - self.p.z) * factor),
        // };
        // return (closest_point.dist(p), factor);

        let ap = *p - self.p;
        // let cross = ap.holy_cross_product(&self.vector);
        // let closest_point = cross.mul(1.0 / self.l);

        let dot_ap_ab = ap.dot(&self.vector);
        let dot_ab_ab = self.vector.len_squared();
        let factor = dot_ap_ab / dot_ab_ab;
        let closest_point = self.p + (self.vector.mul(factor));

        let dist = closest_point.dist(p);
        return (dist, factor);
    }
}
