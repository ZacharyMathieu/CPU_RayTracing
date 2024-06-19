use crate::{
    parameters::RayParameters,
    position::Position,
    sphere::{Sphere, SphereType},
    util::{self, round},
    vector::Vector,
};

fn squared(f: f64) -> f64 {
    return f * f;
}

#[derive(Clone, Copy)]

pub struct Ray {
    pub vector: Vector,
    pub refraction_factor: f64,
    pub x_value: i32,
    pub y_value: i32,
}

impl Ray {
    pub fn new(
        p1: Position,
        p2: Position,
        refraction_factor: f64,
        x_value: i32,
        y_value: i32,
    ) -> Ray {
        return Ray {
            vector: Vector::new(p1, p2),
            refraction_factor: refraction_factor,
            x_value: x_value,
            y_value: y_value,
        };
    }

    pub fn new_turned(
        p: Position,
        d: Position,
        refraction_factor: f64,
        x_value: i32,
        y_value: i32,
        x_angle: f64,
        y_angle: f64,
        z_angle: f64,
    ) -> Ray {
        let mut r = Ray::new(p, d, refraction_factor, x_value, y_value);
        r.turn_x(x_angle);
        r.turn_y(y_angle);
        r.turn_z(z_angle);
        return r;
    }

    fn update_vector(&mut self) {
        self.vector.update();
    }

    pub fn turn_x(&mut self, angle: f64) {
        self.vector.p2.turn_x_around(angle, &self.vector.p1);
        self.update_vector();
    }

    pub fn turn_y(&mut self, angle: f64) {
        self.vector.p2.turn_y_around(angle, &self.vector.p1);
        self.update_vector();
    }

    pub fn turn_z(&mut self, angle: f64) {
        self.vector.p2.turn_z_around(angle, &self.vector.p1);
        self.update_vector();
    }

    pub fn get_position_from_factor(&self, factor: f64) -> Position {
        return self.vector.p1 + (self.vector.as_position()).scaled(factor);
    }

    pub fn factor_distance_from_point(
        &self,
        s: &Sphere,
        ray_parameters: &RayParameters,
    ) -> (f64, bool) {
        // These are the parts of a quadratic equation given by substituting
        // the values of the line (ray) into the equation for the given sphere
        let a: f64 = squared(self.vector.p2.x - self.vector.p1.x)
            + squared(self.vector.p2.y - self.vector.p1.y)
            + squared(self.vector.p2.z - self.vector.p1.z);
        let b: f64 = 2.
            * ((self.vector.p2.x - self.vector.p1.x) * (self.vector.p1.x - s.pos.x)
                + (self.vector.p2.y - self.vector.p1.y) * (self.vector.p1.y - s.pos.y)
                + (self.vector.p2.z - self.vector.p1.z) * (self.vector.p1.z - s.pos.z));
        let c: f64 = squared(s.pos.x)
            + squared(s.pos.y)
            + squared(s.pos.z)
            + squared(self.vector.p1.x)
            + squared(self.vector.p1.y)
            + squared(self.vector.p1.z)
            - 2. * (s.pos.x * self.vector.p1.x
                + s.pos.y * self.vector.p1.y
                + s.pos.z * self.vector.p1.z)
            - squared(s.radius);

        let d = squared(b) - 4. * a * c;
        if d < 0. {
            return (f64::NAN, false);
        }

        let root: f64 = f64::sqrt(d);

        if 0. > round(root + b, 8) {
            return ((-b - root) / (2. * a), true);
        } else if round(root - b, 8) > 0. {
            match s.type_ {
                SphereType::Reflexive => {
                    if ray_parameters.reflect_inside_spheres {
                        return ((-b + root) / (2. * a), false);
                    }
                }
                SphereType::Refractive => {
                    return ((-b + root) / (2. * a), false);
                }
            }
        }

        return (f64::NAN, false);
    }

    pub fn find_collision<'a>(
        &self,
        sphere_vector: &'a Vec<&Sphere>,
        ray_parameters: &RayParameters,
    ) -> Option<((f64, bool), &'a Sphere)> {
        let mut result: Option<((f64, bool), &Sphere)> = Option::None;

        for sphere in sphere_vector.iter() {
            if sphere.is_visible {
                let (ray_factor, is_front): (f64, bool) =
                    self.factor_distance_from_point(&sphere, ray_parameters);

                if !ray_factor.is_nan() {
                    // Check if result is already assigned and if so, override the value if the new factor is smaller
                    match result {
                        None => {
                            result = Option::Some(((ray_factor, is_front), sphere));
                        }
                        Some(((factor, _), _)) => {
                            if ray_factor < factor {
                                result = Option::Some(((ray_factor, is_front), sphere));
                            }
                        }
                    };
                }
            }
        }

        return result;
    }

    pub fn get_deviation(
        &self,
        intersection_factor: f64,
        is_entering: bool,
        sphere: &Sphere,
        ray_parameters: &RayParameters,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Ray {
        match sphere.type_ {
            SphereType::Reflexive => {
                return self.get_reflection(intersection_factor, sphere, ray_parameters, rng);
            }
            SphereType::Refractive => {
                return self.get_refraction(intersection_factor, is_entering, sphere);
            }
        }
    }

    fn get_reflection(
        &self,
        intersection_factor: f64,
        sphere: &Sphere,
        ray_parameters: &RayParameters,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Ray {
        let mut bounced_ray = self.get_perfect_reflection(intersection_factor, sphere);

        bounced_ray.turn_x(util::rand_range(
            rng,
            ray_parameters.min_random_bounce_angle_change * sphere.reflexivity_factor,
            ray_parameters.max_random_bounce_angle_change * sphere.reflexivity_factor,
        ));
        bounced_ray.turn_y(util::rand_range(
            rng,
            ray_parameters.min_random_bounce_angle_change * sphere.reflexivity_factor,
            ray_parameters.max_random_bounce_angle_change * sphere.reflexivity_factor,
        ));
        bounced_ray.turn_z(util::rand_range(
            rng,
            ray_parameters.min_random_bounce_angle_change * sphere.reflexivity_factor,
            ray_parameters.max_random_bounce_angle_change * sphere.reflexivity_factor,
        ));

        return bounced_ray;
    }

    fn get_perfect_reflection(&self, intersection_factor: f64, sphere: &Sphere) -> Ray {
        let intersection = self.get_position_from_factor(intersection_factor);
        let u = intersection - sphere.pos;
        let v = intersection - self.vector.p1;
        let w = u.scaled(-(v.dot(&u) / u.dot(&u)));
        let direction = (intersection + w).scaled(2.) - self.vector.p1;

        return Ray::new(
            intersection,
            direction,
            self.refraction_factor,
            self.x_value,
            self.y_value,
        );
    }

    fn get_refraction(&self, intersection_factor: f64, is_entering: bool, sphere: &Sphere) -> Ray {
        // sin(t1) / sin(t2) = n2 / n1
        // sin(t2) = sin(t1) * (n1 / n2)

        let (n1, n2) = if is_entering {
            (self.refraction_factor, sphere.refractivity_factor)
        } else {
            (sphere.refractivity_factor, self.refraction_factor)
        };

        let intersection: Position = self.get_position_from_factor(intersection_factor);
        let normal_sphere: Position = intersection - sphere.pos;
        let (normal, normal2) = if is_entering {
            (-normal_sphere, normal_sphere)
        } else {
            (normal_sphere, -normal_sphere)
        };

        let incident: Position = -self.vector.as_position();

        let A1 = incident.angle(&normal);

        if A1.sin() > n2 / n1 {
            return self.get_perfect_reflection(intersection_factor, sphere);
        }

        let A2: f64 = f64::asin((n1 * f64::sin(A1)) / n2);

        let a: f64 = f64::sqrt(
            1. / (incident.dot(&incident) - (incident.dot(&normal).powf(2.) / normal.dot(&normal))),
        );
        let b: f64 = -(a * incident.dot(&normal));

        let U = incident.scaled(a) + normal.scaled(b);
        let V2 = -(normal2.scaled(f64::cos(A2)) + U.scaled(f64::sin(A2)));

        return Ray::new(
            intersection,
            intersection + V2,
            if is_entering {
                sphere.refractivity_factor
            } else {
                1.
            },
            self.x_value,
            self.y_value,
        );
    }
}
