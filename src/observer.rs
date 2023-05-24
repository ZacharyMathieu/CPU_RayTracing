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
            pos: parameters.observer_default_position,
            hor_angle: 0.0,
            ver_angle: 0.0,
            rays: Vec::new(),
        };
        obs.generate_rays(parameters);
        return obs;
    }

    fn generate_rays(&mut self, parameters: &Parameters) {
        self.rays.clear();

        for x in parameters.min_hor_ray_value..parameters.max_hor_ray_value as i32 {
            for y in parameters.min_ver_ray_value..parameters.max_ver_ray_value as i32 {
                let r = Ray::new_turned(
                    self.pos,
                    Position {
                        x: parameters.observer_look_vector_distance,
                        y: x as f64,
                        z: y as f64,
                    },
                    x - parameters.min_hor_ray_value,
                    y - parameters.min_ver_ray_value,
                    self.hor_angle,
                    self.ver_angle,
                );
                self.rays.push(r);
            }
        }
    }

    pub fn turn_hor(&mut self, angle: f64, parameters: &Parameters) {
        self.hor_angle += angle;
        self.generate_rays(parameters);
    }

    pub fn turn_ver(&mut self, angle: f64, parameters: &Parameters) {
        self.ver_angle += angle;
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
