use crate::{
    object::Object,
    parameters::RayParameters,
    polygon::Polygon,
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
        origin: Position,
        direction: Position,
        refraction_factor: f64,
        x_value: i64,
        y_value: i64,
    ) -> Ray {
        return Ray {
            vector: Vector::new(origin, direction),
            refraction_factor,
            x_value,
            y_value,
        };
    }

    pub fn new_turned(
        origin: Position,
        direction: Position,
        refraction_factor: f64,
        x_value: i64,
        y_value: i64,
        x_angle: f64,
        y_angle: f64,
        z_angle: f64,
    ) -> Ray {
        let mut r = Ray::new(origin, direction, refraction_factor, x_value, y_value);
        r.turn(Position {
            x: x_angle,
            y: y_angle,
            z: z_angle,
        });
        return r;
    }

    fn update_vector(&mut self) {
        self.vector.update();
    }

    pub fn turn(&mut self, angle: Position) {
        self.vector.direction.turn(angle);
        self.update_vector();
    }

    pub fn get_position_from_factor(&self, factor: f64) -> Position {
        return self.vector.origin + (self.vector.as_position()).scaled(factor);
    }

    pub fn factor_distance_from_object(
        &self,
        o: &Object,
        ray_parameters: &RayParameters,
    ) -> (f64, bool) {
        match o {
            Object::Sphere(s) => {
                let c0 = s.pos;
                let cr = s.radius;
                let r0 = self.vector.origin;
                let r1 = self.vector.direction;
                let a = squared(r1.x) + squared(r1.y) + squared(r1.z);
                let b = 2. * r1.x * r0.x - 2. * r1.x * c0.x + 2. * r1.y * r0.y - 2. * r1.y * c0.y
                    + 2. * r1.z * r0.z
                    - 2. * r1.z * c0.z;
                let c = squared(r0.x) - 2. * r0.x * c0.x + squared(c0.x) + squared(r0.y)
                    - 2. * r0.y * c0.y
                    + squared(c0.y)
                    + squared(r0.z)
                    - 2. * r0.z * c0.z
                    + squared(c0.z)
                    - squared(cr);

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
                let r0: Position = self.vector.origin;
                let r1: Position = self.vector.direction;

                let n: Position = p.get_normal();
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
                    / (p1.x * p2.y + p1.y * p2.x);
                let f1 = (i.x - p0.x - f2 * p2.x) / p1.x;
                if 0. < f1 && 0. < f2 && f1 + f2 < 1. {
                    return (r, n * r1 > 0.);
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
                SurfaceType::Reflexive => {
                    self.get_reflection(intersection_factor, is_entering, object)
                }
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

        ray.turn(Position {
            x: util::rand_range(
                rng,
                ray_parameters.min_random_bounce_angle_change * smoothness_factor,
                ray_parameters.max_random_bounce_angle_change * smoothness_factor,
            ),
            y: util::rand_range(
                rng,
                ray_parameters.min_random_bounce_angle_change * smoothness_factor,
                ray_parameters.max_random_bounce_angle_change * smoothness_factor,
            ),
            z: util::rand_range(
                rng,
                ray_parameters.min_random_bounce_angle_change * smoothness_factor,
                ray_parameters.max_random_bounce_angle_change * smoothness_factor,
            ),
        });
        // ray.turn_x(util::rand_range(
        //     rng,
        //     ray_parameters.min_random_bounce_angle_change * smoothness_factor,
        //     ray_parameters.max_random_bounce_angle_change * smoothness_factor,
        // ));
        // ray.turn_y(util::rand_range(
        //     rng,
        //     ray_parameters.min_random_bounce_angle_change * smoothness_factor,
        //     ray_parameters.max_random_bounce_angle_change * smoothness_factor,
        // ));
        // ray.turn_z(util::rand_range(
        //     rng,
        //     ray_parameters.min_random_bounce_angle_change * smoothness_factor,
        //     ray_parameters.max_random_bounce_angle_change * smoothness_factor,
        // ));

        return ray;
    }

    fn get_normal(intersection: &Position, is_entering: bool, object: &Object) -> Position {
        match object {
            Object::Sphere(sphere) => {
                return *intersection - sphere.pos;
            }
            Object::Polygon(polygon) => {
                if is_entering {
                    return -polygon.get_normal();
                } else {
                    return polygon.get_normal();
                }
            }
        }
    }

    fn get_reflection(&self, intersection_factor: f64, is_entering: bool, object: &Object) -> Ray {
        let intersection = self.get_position_from_factor(intersection_factor);
        let n = Ray::get_normal(&intersection, is_entering, object);
        let d = self.vector.direction;
        // let v = intersection - self.vector.origin;
        // let w = u * -((v * u) / (u * u));
        // let direction = (intersection + w) * 2. - self.vector.origin;
        let direction = d - n * (2. * (d * n));

        return Ray::new(
            intersection,
            direction,
            self.refraction_factor,
            self.x_value,
            self.y_value,
        );
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
                    return self.get_reflection(intersection_factor, is_entering, object);
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
