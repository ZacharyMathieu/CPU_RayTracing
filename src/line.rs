use crate::position::Position;

pub struct Line<'a> {
    pub p1: &'a Position,
    pub p2: &'a Position,
}

impl<'a> Line<'a> {
    pub fn dist_point(&self, p0: &Position) -> f64 {
        let factor =
            -((self.p1.x - p0.x) * (self.p2.x - self.p1.x)) / (self.p2.x - self.p1.x).powf(2.0);
        let closest_point = Position {
            x: self.p1.x + ((self.p2.x - self.p1.x) * factor),
            y: self.p1.y + ((self.p2.y - self.p1.y) * factor),
            z: self.p1.z + ((self.p2.z - self.p1.z) * factor),
        };
        if factor < 0.0 {
            return -closest_point.dist(p0);
        } else {
            return closest_point.dist(p0);
        }
    }
}
