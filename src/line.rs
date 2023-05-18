use crate::position::Position;

pub struct Line<'a> {
    pub p1: &'a Position,
    pub p2: &'a Position,
}

impl<'a> Line<'a> {
    pub fn dist_point(&self, p: &Position) -> f64 {
        return ((((self.p2.x - self.p1.x) * (self.p1.y - p.y))
            - ((self.p2.y - self.p1.y) * (self.p1.x - p.x)))
            .abs())
            / (self.p1.dist(&self.p2));
    }
}
