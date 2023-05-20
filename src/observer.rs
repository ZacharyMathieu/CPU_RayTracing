use crate::{parameters::Parameters, position::Position, ray::Ray};

pub struct Observer {
    pub ray: Ray,
    pub rays: Vec<Ray>,
}

impl Observer {
    pub fn default(parameters: &Parameters) -> Observer {
        let mut obs = Observer {
            ray: Ray::new(
                Position {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Position {
                    x: parameters.observer_look_vector_distance,
                    y: 0.0,
                    z: 0.0,
                },
                0,
                0,
            ),
            rays: Vec::new(),
        };
        obs.generate_rays(parameters);
        return obs;
    }

    fn generate_rays(&mut self, parameters: &Parameters) {
        self.rays.clear();

        let hor_step = (parameters.max_hor_angle - parameters.min_hor_angle)
            / (parameters.hor_rays - 1) as f64;
        let ver_step = (parameters.max_ver_angle - parameters.min_ver_angle)
            / (parameters.ver_rays - 1) as f64;

        let mut hor_angle: f64 = parameters.min_hor_angle;
        let mut ver_angle: f64;

        for x in 0..parameters.hor_rays as i32 {
            ver_angle = parameters.min_ver_angle;
            for y in 0..parameters.ver_rays as i32 {
                self.rays.push(self.ray.generate_turned(
                    hor_angle.clone(),
                    ver_angle.clone(),
                    x,
                    y,
                ));
                ver_angle += ver_step;
            }
            hor_angle += hor_step;
        }
    }

    pub fn turn_hor(&mut self, angle: f64, parameters: &Parameters) {
        self.ray.turn_hor(angle);
        self.generate_rays(parameters);
    }

    pub fn turn_ver(&mut self, angle: f64, parameters: &Parameters) {
        self.ray.turn_ver(angle);
        self.generate_rays(parameters);
    }
}
