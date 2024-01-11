use rand::random;

use crate::{parameters::RayParameters, position::Position, sphere::Sphere, util};

fn squared(f: f64) -> f64 {
    return f * f;
}

#[derive(Clone, Copy)]
pub struct Ray {
    pub p1: Position,
    pub p2: Position,
    vector: Position,
    pub length: f64,
    pub x_value: i32,
    pub y_value: i32,
}

impl Ray {
    pub fn new(p1: Position, p2: Position, x_value: i32, y_value: i32) -> Ray {
        let vector = p2 - p1;
        let l = p1.dist(&p2);
        return Ray {
            p1,
            p2,
            vector: vector,
            length: l,
            x_value: x_value,
            y_value: y_value,
        };
    }

    pub fn new_turned(
        p: Position,
        d: Position,
        x_value: i32,
        y_value: i32,
        x_angle: f64,
        y_angle: f64,
        z_angle: f64,
    ) -> Ray {
        let mut r = Ray::new(p, d, x_value, y_value);
        r.turn_x(x_angle);
        r.turn_y(y_angle);
        r.turn_z(z_angle);
        return r;
    }

    fn update_vector_and_len(&mut self) {
        self.vector = self.p2 - self.p1;
        self.length = self.p1.dist(&self.p2);
    }

    pub fn turn_x(&mut self, angle: f64) {
        self.p2.turn_x_around(angle, &self.p1);
        self.update_vector_and_len();
    }

    pub fn turn_y(&mut self, angle: f64) {
        self.p2.turn_y_around(angle, &self.p1);
        self.update_vector_and_len();
    }

    pub fn turn_z(&mut self, angle: f64) {
        self.p2.turn_z_around(angle, &self.p1);
        self.update_vector_and_len();
    }

    pub fn get_position_from_factor(&self, factor: f64) -> Position {
        return self.p1 + (self.vector).scaled(factor);
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

        let d = squared(b) - 4.0 * a * c;
        if d < 0.0 {
            return f64::NAN;
        }

        let root = f64::sqrt(d);
        let ret: f64;

        if root > b {
            ret = (-b - root) / (2.0 * a);
        } else {
            ret = (-b + root) / (2.0 * a);
        }

        // Remove the points behind the observer
        if ret <= 0.0 {
            return f64::NAN;
        }

        return ret;
    }

    pub fn find_collision<'a>(&self, sphere_vector: &'a Vec<Sphere>) -> Option<(f64, &'a Sphere)> {
        let mut result: Option<(f64, &Sphere)> = Option::None;

        for sphere in sphere_vector.iter() {
            let ray_factor = self.factor_distance_from_point(&sphere);

            if !ray_factor.is_nan() {
                match result {
                    None => {
                        result = Option::Some((ray_factor, sphere));
                    }
                    Some((factor, _)) => {
                        if ray_factor < factor {
                            result = Option::Some((ray_factor, sphere));
                        }
                    }
                };
            }
        }

        return result;
    }

    pub fn get_bounce(
        &self,
        intersection_factor: f64,
        sphere: &Sphere,
        ray_parameters: &RayParameters,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Ray {
        if random::<f64>() < sphere.reflexivity_factor {
            return self.bounce_reflect(intersection_factor, sphere);
        } else {
            return self.bounce_random(intersection_factor, sphere, ray_parameters, rng);
        }
    }

    fn bounce_reflect(&self, intersection_factor: f64, sphere: &Sphere) -> Ray {
        let intersection = self.get_position_from_factor(intersection_factor);
        let u = intersection - sphere.pos;
        let v = intersection - self.p1;
        let w = u.scaled(-(v.dot_product(&u) / u.dot_product(&u)));
        let direction = (intersection + w).scaled(2.0) - self.p1;

        return Ray::new(intersection, direction, self.x_value, self.y_value);
    }

    fn bounce_random(
        &self,
        intersection_factor: f64,
        sphere: &Sphere,
        ray_parameters: &RayParameters,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Ray {
        let mut bounced_ray = self.bounce_reflect(intersection_factor, sphere);

        bounced_ray.turn_x(util::rand_range(
            rng,
            ray_parameters.min_random_bounce_angle_change,
            ray_parameters.max_random_bounce_angle_change,
        ));
        bounced_ray.turn_y(util::rand_range(
            rng,
            ray_parameters.min_random_bounce_angle_change,
            ray_parameters.max_random_bounce_angle_change,
        ));
        bounced_ray.turn_z(util::rand_range(
            rng,
            ray_parameters.min_random_bounce_angle_change,
            ray_parameters.max_random_bounce_angle_change,
        ));

        return bounced_ray;
    }
}
