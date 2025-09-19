use crate::{
    object::Object,
    parameters::RayParameters,
    position::Position,
    surface_type::SurfaceType,
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
    pub x_value: i64,
    pub y_value: i64,
}

impl Ray {
    pub fn new(
        p1: Position,
        p2: Position,
        refraction_factor: f64,
        x_value: i64,
        y_value: i64,
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
        x_value: i64,
        y_value: i64,
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

    pub fn factor_distance_from_object(
        &self,
        o: &Object,
        ray_parameters: &RayParameters,
    ) -> (f64, bool) {
        match o {
            Object::Sphere(s) => {
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
                        SurfaceType::Reflexive => {
                            if ray_parameters.reflect_inside_spheres {
                                return ((-b + root) / (2. * a), false);
                            }
                        }
                        SurfaceType::Refractive => {
                            return ((-b + root) / (2. * a), false);
                        }
                    }
                }

                return (f64::NAN, false);
            }
            Object::Polygon(p) => {
                let p0: Position = p.pos;
                let p1: Position = p.v1;
                let p2: Position = p.v2;
                let r0: Position = self.vector.p1;
                let r1: Position = self.vector.p2;

                let n: Position = p1.cross(&p2);
                let det = n * r1;
                if det == 0. {
                    return (f64::NAN, false);
                }

                let r: f64 = (n * (p0 - r0)) / det;
                if r <= 0. {
                    return (f64::NAN, false);
                }

                let i = r0 + r1 * r;

                let f2 = (i.y * p1.x - i.x * p1.y + p0.x * p1.y - p0.y * p1.x)
                    / (p1.y * p2.x + p1.x * p2.y);
                let f1 = (i.x - p0.x - f2 * p2.x) / p1.x;
                if 0. <= f1 && 0. <= f2 && f1 + f2 <= 1. {
                    return (r, false);
                }
                return (f64::NAN, false);
            }
        }
    }

    pub fn find_collision<'a>(
        &self,
        object_vector: &'a Vec<&Object>,
        ray_parameters: &RayParameters,
    ) -> Option<((f64, bool), &'a Object)> {
        let mut result: Option<((f64, bool), &Object)> = Option::None;

        for object in object_vector.iter() {
            if object.is_visible() {
                let (ray_factor, is_front): (f64, bool) =
                    self.factor_distance_from_object(&object, ray_parameters);

                if !ray_factor.is_nan() {
                    // Check if result is already assigned and if so, override the value if the new factor is smaller
                    match result {
                        None => {
                            result = Option::Some(((ray_factor, is_front), object));
                        }
                        Some(((factor, _), _)) => {
                            if ray_factor < factor {
                                result = Option::Some(((ray_factor, is_front), object));
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
        object: &Object,
        ray_parameters: &RayParameters,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Ray {
        return self.apply_smoothness(
            match object.type_() {
                SurfaceType::Reflexive => self.get_reflection(intersection_factor, object),
                SurfaceType::Refractive => {
                    self.get_refraction(intersection_factor, is_entering, object)
                }
            },
            object,
            ray_parameters,
            rng,
        );
    }

    fn apply_smoothness(
        &self,
        mut ray: Ray,
        sphere: &Object,
        ray_parameters: &RayParameters,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Ray {
        let smoothness_factor: f64 = 1. - sphere.smoothness();

        ray.turn_x(util::rand_range(
            rng,
            ray_parameters.min_random_bounce_angle_change * smoothness_factor,
            ray_parameters.max_random_bounce_angle_change * smoothness_factor,
        ));
        ray.turn_y(util::rand_range(
            rng,
            ray_parameters.min_random_bounce_angle_change * smoothness_factor,
            ray_parameters.max_random_bounce_angle_change * smoothness_factor,
        ));
        ray.turn_z(util::rand_range(
            rng,
            ray_parameters.min_random_bounce_angle_change * smoothness_factor,
            ray_parameters.max_random_bounce_angle_change * smoothness_factor,
        ));

        return ray;
    }

    fn get_reflection(&self, intersection_factor: f64, object: &Object) -> Ray {
        match object {
            Object::Sphere(sphere) => {
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
            Object::Polygon(polygon) => {
                return *self;
            }
        }
    }

    // I hope I never have to debug this...
    fn get_refraction(&self, intersection_factor: f64, is_entering: bool, object: &Object) -> Ray {
        match object {
            Object::Sphere(sphere) => {
                let refractivity_index = object.refractivity_index();
                let (n1, n2) = if is_entering {
                    (self.refraction_factor, refractivity_index)
                } else {
                    (refractivity_index, self.refraction_factor)
                };

                let intersection: Position = self.get_position_from_factor(intersection_factor);
                let normal_sphere: Position =
                    (self.get_position_from_factor(intersection_factor) - sphere.pos).normalized();
                let (normal, normal2) = if is_entering {
                    (-normal_sphere, normal_sphere)
                } else {
                    (normal_sphere, -normal_sphere)
                };

                let incident: Position = -self.vector.as_position().normalized();

                let angle_incident = incident.angle(&normal);

                if angle_incident.sin() > n2 / n1 {
                    return self.get_reflection(intersection_factor, object);
                }

                let angle_exit: f64 = f64::asin((n1 * f64::sin(angle_incident)) / n2);

                let a: f64 = f64::sqrt(
                    1. / (incident.dot(&incident)
                        - (incident.dot(&normal).powf(2.) / normal.dot(&normal))),
                );

                let exit = -(normal2.scaled(f64::cos(angle_exit))
                    + (incident.scaled(a) + normal.scaled(-(a * incident.dot(&normal))))
                        .scaled(f64::sin(angle_exit)));

                return Ray::new(
                    intersection,
                    intersection + exit,
                    if is_entering { refractivity_index } else { 1. },
                    self.x_value,
                    self.y_value,
                );
            }
            Object::Polygon(polygon) => {
                return *self;
            }
        }
    }
}
