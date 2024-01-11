use crate::{parameters::Parameters, position::Position, ray::Ray};

pub struct Observer {
    pub pos: Position,
    pub hor_angle: f64,
    pub ver_angle: f64,
    pub rays: Vec<Ray>,
}

impl Observer {
    pub fn default(parameters: &Parameters) -> Observer {
        let mut obs = Observer {
            pos: parameters.observer_parameters.default_position.clone(),
            hor_angle: 0.0,
            ver_angle: 0.0,
            rays: Vec::new(),
        };
        obs.generate_rays(parameters);
        return obs;
    }

    fn generate_rays(&mut self, parameters: &Parameters) {
        self.rays.clear();

        for x in
            parameters.ray_parameters.min_hor_value..parameters.ray_parameters.max_hor_value as i32
        {
            for y in parameters.ray_parameters.min_ver_value
                ..parameters.ray_parameters.max_ver_value as i32
            {
                let r = Ray::new_turned(
                    self.pos.clone(),
                    Position {
                        x: parameters.observer_parameters.look_vector_distance,
                        y: x as f64,
                        z: y as f64,
                    },
                    x - parameters.ray_parameters.min_hor_value,
                    y - parameters.ray_parameters.min_ver_value,
                    0.,
                    self.ver_angle,
                    self.hor_angle,
                );
                self.rays.push(r);
            }
        }
    }

    fn limit_angle(current: f64, min_angle: f64, max_angle: f64, loop_angle: bool) -> f64 {
        if current < min_angle {
            if loop_angle {
                return max_angle - (min_angle - current);
            } else {
                return min_angle;
            }
        } else if current > max_angle {
            if loop_angle {
                return min_angle + (current - max_angle);
            } else {
                return max_angle;
            }
        } else {
            return current;
        }
    }

    pub fn turn_hor(&mut self, angle: f64, parameters: &Parameters) {
        self.hor_angle += angle;

        self.hor_angle = Self::limit_angle(
            self.hor_angle,
            parameters.observer_parameters.min_hor_angle,
            parameters.observer_parameters.max_hor_angle,
            parameters.observer_parameters.hor_angle_loop,
        );

        self.generate_rays(parameters);
    }

    pub fn turn_ver(&mut self, angle: f64, parameters: &Parameters) {
        self.ver_angle += angle;

        self.ver_angle = Self::limit_angle(
            self.ver_angle,
            parameters.observer_parameters.min_ver_angle,
            parameters.observer_parameters.max_ver_angle,
            parameters.observer_parameters.ver_angle_loop,
        );

        self.generate_rays(parameters);
    }

    pub fn move_forward(&mut self, dist: f64, parameters: &Parameters) {
        self.pos.x += self.hor_angle.cos() * self.ver_angle.cos() * dist;
        self.pos.y += self.hor_angle.sin() * dist;
        self.pos.z += self.ver_angle.sin() * dist;
        self.generate_rays(parameters);
    }

    pub fn move_hor(&mut self, dist: f64, parameters: &Parameters) {
        self.pos.x -= self.hor_angle.sin() * dist;
        self.pos.y += self.hor_angle.cos() * dist;
        self.generate_rays(parameters);
    }

    pub fn move_ver(&mut self, dist: f64, parameters: &Parameters) {
        self.pos.z += dist;
        self.generate_rays(parameters);
    }
}
