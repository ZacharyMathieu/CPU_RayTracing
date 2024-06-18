use crate::{
    frame::Frame,
    parameters::{ObserverParameters, Parameters, RayParameters},
    position::Position,
    ray::Ray,
    ray_trace::RayTrace,
    speed::Speed,
    sphere::Sphere,
};
use rayon::prelude::*;

pub struct Observer {
    pub body: Sphere,
    pub hor_angle: f64,
    pub ver_angle: f64,
    pub rays: Vec<Ray>,
    pub accumulation_mode: bool,
    frame_stack: Vec<Frame>,
    slow_speed_mode: bool,
}

impl Observer {
    pub fn default(parameters: &Parameters) -> Observer {
        let mut obs = Observer {
            hor_angle: 0.,
            ver_angle: 0.,
            rays: Vec::new(),
            accumulation_mode: false,
            frame_stack: Vec::new(),
            slow_speed_mode: false,
            body: parameters.observer_parameters.default_body.clone(),
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
                    self.body.pos.clone(),
                    Position {
                        x: parameters.observer_parameters.look_vector_distance,
                        y: x as f64,
                        z: y as f64,
                    },
                    1.,
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

    fn generate_ray_traces(&self, ray_parameters: &RayParameters) -> Vec<RayTrace> {
        let mut ray_traces: Vec<RayTrace> = Vec::new();

        self.rays.iter().for_each(|ray| {
            ray_traces.push(RayTrace::new(ray, ray_parameters));
        });

        return ray_traces;
    }

    fn trace_parallel(
        &self,
        ray_parameters: &RayParameters,
        sphere_vector: &Vec<&Sphere>,
    ) -> Vec<RayTrace> {
        let mut ray_traces: Vec<RayTrace> = self.generate_ray_traces(ray_parameters);

        // Parallel ray casting
        ray_traces
            .par_iter_mut()
            .for_each(|trace: &mut RayTrace<'_>| {
                trace.trace(sphere_vector, ray_parameters);
            });

        return ray_traces;
    }

    pub fn get_next_frame(
        &mut self,
        ray_parameters: &RayParameters,
        sphere_vector: &Vec<&Sphere>,
    ) -> Frame {
        let traces: Vec<RayTrace> = self.trace_parallel(ray_parameters, sphere_vector);

        let frame: Frame = Frame::create_from_ray_trace(traces);

        if self.accumulation_mode {
            self.frame_stack.push(frame);

            return Frame::accumulate_frames(&self.frame_stack);
        }

        return frame;
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

    fn apply_slow_mode(&self, value: f64, observer_parameters: &ObserverParameters) -> f64 {
        if self.slow_speed_mode {
            return value * observer_parameters.slow_mode_factor;
        } else {
            return value;
        };
    }

    pub fn turn_hor(&mut self, angle: f64, parameters: &Parameters) {
        self.hor_angle += self.apply_slow_mode(angle, &parameters.observer_parameters);

        self.hor_angle = Self::limit_angle(
            self.hor_angle,
            parameters.observer_parameters.min_hor_angle,
            parameters.observer_parameters.max_hor_angle,
            parameters.observer_parameters.hor_angle_loop,
        );

        self.generate_rays(parameters);
    }

    pub fn turn_ver(&mut self, angle: f64, parameters: &Parameters) {
        self.ver_angle += self.apply_slow_mode(angle, &parameters.observer_parameters);

        self.ver_angle = Self::limit_angle(
            self.ver_angle,
            parameters.observer_parameters.min_ver_angle,
            parameters.observer_parameters.max_ver_angle,
            parameters.observer_parameters.ver_angle_loop,
        );

        self.generate_rays(parameters);
    }

    pub fn move_forward(&mut self, dist: f64, parameters: &Parameters) {
        self.move_(
            Speed {
                x: self.hor_angle.cos() * self.ver_angle.cos() * dist,
                y: self.hor_angle.sin() * dist,
                z: self.ver_angle.sin() * dist,
            },
            &parameters.observer_parameters,
        );

        self.generate_rays(parameters);
    }

    pub fn move_hor(&mut self, dist: f64, parameters: &Parameters) {
        self.move_(
            Speed {
                x: -self.hor_angle.sin() * dist,
                y: self.hor_angle.cos() * dist,
                z: 0.,
            },
            &parameters.observer_parameters,
        );

        self.generate_rays(parameters);
    }

    pub fn move_ver(&mut self, dist: f64, parameters: &Parameters) {
        self.move_(
            Speed {
                x: 0.,
                y: 0.,
                z: dist,
            },
            &parameters.observer_parameters,
        );

        self.generate_rays(parameters);
    }

    fn move_(&mut self, speed: Speed, observer_parameters: &ObserverParameters) {
        self.body.pos.x += self.apply_slow_mode(speed.x, &observer_parameters);
        self.body.pos.y += self.apply_slow_mode(speed.y, &observer_parameters);
        self.body.pos.z += self.apply_slow_mode(speed.z, &observer_parameters);
    }

    pub fn reset_position(&mut self, parameters: &Parameters) {
        self.body.pos = parameters.observer_parameters.default_body.pos.clone();
        self.body.is_visible = parameters.observer_parameters.default_body.is_visible;

        self.generate_rays(parameters);
    }

    pub fn switch_accumulation_mode(&mut self) {
        self.accumulation_mode = !self.accumulation_mode;

        if !self.accumulation_mode {
            self.frame_stack = Vec::new();
        }
    }

    pub fn switch_visibility(&mut self) {
        self.body.is_visible = !self.body.is_visible;
    }

    pub fn slow_speed_mode(&mut self) {
        self.slow_speed_mode = true;
    }

    pub fn normal_speed_mode(&mut self) {
        self.slow_speed_mode = false;
    }
}
