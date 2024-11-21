use sdl2::pixels::Color;
use std::env;
use std::fs;

use crate::{
    position::Position,
    speed::Speed,
    sphere::{Sphere, SphereType},
};

pub enum SphereGenerationMode {
    Hardcoded,
    Random,
    InLine,
}

pub struct ObserverParameters {
    pub look_vector_distance: f64,
    pub look_up_angle: f64,
    pub look_down_angle: f64,
    pub look_left_angle: f64,
    pub look_right_angle: f64,
    pub min_hor_angle: f64,
    pub max_hor_angle: f64,
    pub hor_angle_loop: bool,
    pub min_ver_angle: f64,
    pub max_ver_angle: f64,
    pub ver_angle_loop: bool,
    pub move_forward_distance: f64,
    pub move_backward_distance: f64,
    pub move_right_distance: f64,
    pub move_left_distance: f64,
    pub move_up_distance: f64,
    pub move_down_distance: f64,
    pub slow_mode_factor: f64,
    pub default_body: Sphere,
}

impl ObserverParameters {
    fn update_from_json(&mut self, data: &serde_json::Value) {
        self.look_vector_distance = *data["look_vector_distance"]
            .as_f64()
            .get_or_insert(self.look_vector_distance);
        self.look_up_angle = *data["look_up_angle"]
            .as_f64()
            .get_or_insert(self.look_up_angle);
        self.look_up_angle = *data["look_up_angle"]
            .as_f64()
            .get_or_insert(self.look_up_angle);
        self.look_down_angle = *data["look_down_angle"]
            .as_f64()
            .get_or_insert(self.look_down_angle);
        self.look_left_angle = *data["look_left_angle"]
            .as_f64()
            .get_or_insert(self.look_left_angle);
        self.look_right_angle = *data["look_right_angle"]
            .as_f64()
            .get_or_insert(self.look_right_angle);
        self.min_hor_angle = *data["min_hor_angle"]
            .as_f64()
            .get_or_insert(self.min_hor_angle);
        self.max_hor_angle = *data["max_hor_angle"]
            .as_f64()
            .get_or_insert(self.max_hor_angle);
        self.hor_angle_loop = *data["hor_angle_loop"]
            .as_bool()
            .get_or_insert(self.hor_angle_loop);
        self.min_ver_angle = *data["min_ver_angle"]
            .as_f64()
            .get_or_insert(self.min_ver_angle);
        self.max_ver_angle = *data["max_ver_angle"]
            .as_f64()
            .get_or_insert(self.max_ver_angle);
        self.ver_angle_loop = *data["ver_angle_loop"]
            .as_bool()
            .get_or_insert(self.ver_angle_loop);
        self.move_forward_distance = *data["move_forward_distance"]
            .as_f64()
            .get_or_insert(self.move_forward_distance);
        self.move_backward_distance = *data["move_backward_distance"]
            .as_f64()
            .get_or_insert(self.move_backward_distance);
        self.move_right_distance = *data["move_right_distance"]
            .as_f64()
            .get_or_insert(self.move_right_distance);
        self.move_left_distance = *data["move_left_distance"]
            .as_f64()
            .get_or_insert(self.move_left_distance);
        self.move_up_distance = *data["move_up_distance"]
            .as_f64()
            .get_or_insert(self.move_up_distance);
        self.move_down_distance = *data["move_down_distance"]
            .as_f64()
            .get_or_insert(self.move_down_distance);
        self.slow_mode_factor = *data["slow_mode_factor"]
            .as_f64()
            .get_or_insert(self.slow_mode_factor);
    }
}

pub struct RayParameters {
    pub min_hor_value: i64,
    pub max_hor_value: i64,
    pub min_ver_value: i64,
    pub max_ver_value: i64,
    pub min_pixel_factor: f64,
    pub fog_factor: f64,
    pub background_color: Color,
    pub background_light_factor: f64,
    pub reflect_background: bool,
    pub bounce_count: u64,
    pub bounce_color_reflection_factor: f64,
    pub min_random_bounce_angle_change: f64,
    pub max_random_bounce_angle_change: f64,
    pub reflect_inside_spheres: bool,
}

impl RayParameters {
    fn update_from_json(&mut self, data: &serde_json::Value) {
        self.min_hor_value = *data["min_hor_value"]
            .as_i64()
            .get_or_insert(self.min_hor_value);
        self.max_hor_value = *data["max_hor_value"]
            .as_i64()
            .get_or_insert(self.max_hor_value);
        self.min_ver_value = *data["min_ver_value"]
            .as_i64()
            .get_or_insert(self.min_ver_value);
        self.max_ver_value = *data["max_ver_value"]
            .as_i64()
            .get_or_insert(self.max_ver_value);
        self.min_pixel_factor = *data["min_pixel_factor"]
            .as_f64()
            .get_or_insert(self.min_pixel_factor);
        self.fog_factor = *data["fog_factor"].as_f64().get_or_insert(self.fog_factor);
        let background_color: &Vec<serde_json::Value> = data["background_color"]
            .as_array()
            .expect("Invalid background color");
        self.background_color = Color::RGB(
            *background_color[0]
                .as_u64()
                .get_or_insert(self.background_color.r as u64) as u8,
            *background_color[1]
                .as_u64()
                .get_or_insert(self.background_color.g as u64) as u8,
            *background_color[2]
                .as_u64()
                .get_or_insert(self.background_color.b as u64) as u8,
        );
        println!("bg color r: {}", self.background_color.r);
        println!("bg color g: {}", self.background_color.g);
        println!("bg color b: {}", self.background_color.b);
        self.background_light_factor = *data["background_light_factor"]
            .as_f64()
            .get_or_insert(self.background_light_factor);
        self.reflect_background = *data["reflect_background"]
            .as_bool()
            .get_or_insert(self.reflect_background);
        self.bounce_count = *data["bounce_count"]
            .as_u64()
            .get_or_insert(self.bounce_count);
        self.bounce_color_reflection_factor = *data["bounce_color_reflection_factor"]
            .as_f64()
            .get_or_insert(self.bounce_color_reflection_factor);
        self.min_random_bounce_angle_change = *data["min_random_bounce_angle_change"]
            .as_f64()
            .get_or_insert(self.min_random_bounce_angle_change);
        self.max_random_bounce_angle_change = *data["max_random_bounce_angle_change"]
            .as_f64()
            .get_or_insert(self.max_random_bounce_angle_change);
        self.reflect_inside_spheres = *data["reflect_inside_spheres"]
            .as_bool()
            .get_or_insert(self.reflect_inside_spheres);
    }
}

pub struct SphereParameters {
    pub generation_mode: SphereGenerationMode,
    pub sphere_type: SphereType,
    pub sphere_count: u64,
    pub min_radius: f64,
    pub max_radius: f64,
    pub min_light_factor: f64,
    pub max_light_factor: f64,
    pub min_smoothness: f64,
    pub max_smoothness: f64,
    pub min_refractivity_index: f64,
    pub max_refractivity_index: f64,
}

pub struct PhysicsParameters {
    pub g: f64,
    pub enabled: bool,
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
    pub min_z: f64,
    pub max_z: f64,
    pub min_vx: f64,
    pub max_vx: f64,
    pub min_vy: f64,
    pub max_vy: f64,
    pub min_vz: f64,
    pub max_vz: f64,
}

impl PhysicsParameters {
    fn update_from_json(&mut self, data: &serde_json::Value) {
        self.g = *data["g"].as_f64().get_or_insert(self.g);
        self.enabled = *data["enabled"].as_bool().get_or_insert(self.enabled);
        self.min_x = *data["min_x"].as_f64().get_or_insert(self.min_x);
        self.max_x = *data["max_x"].as_f64().get_or_insert(self.max_x);
        self.min_y = *data["min_y"].as_f64().get_or_insert(self.min_y);
        self.max_y = *data["max_y"].as_f64().get_or_insert(self.max_y);
        self.min_z = *data["min_z"].as_f64().get_or_insert(self.min_z);
        self.max_z = *data["max_z"].as_f64().get_or_insert(self.max_z);
        self.min_vx = *data["min_vx"].as_f64().get_or_insert(self.min_vx);
        self.max_vx = *data["max_vx"].as_f64().get_or_insert(self.max_vx);
        self.min_vy = *data["min_vy"].as_f64().get_or_insert(self.min_vy);
        self.max_vy = *data["max_vy"].as_f64().get_or_insert(self.max_vy);
        self.min_vz = *data["min_vz"].as_f64().get_or_insert(self.min_vz);
        self.max_vz = *data["max_vz"].as_f64().get_or_insert(self.max_vz);
    }
}

pub struct Parameters {
    pub parameter_file_path: String,
    pub frame_period_ms: u64,
    pub display_scale: f64,
    pub observer_parameters: ObserverParameters,
    pub ray_parameters: RayParameters,
    pub sphere_parameters: Vec<SphereParameters>,
    pub physics_parameters: PhysicsParameters,
}

impl Parameters {
    pub fn default() -> Parameters {
        let width: i64 = 128;
        let height: i64 = 128;
        let look_angle = 0.1;
        let move_distance = 0.5;
        let physics_bounds_value = 20.;
        let speed_bounds_value = 0.0025;
        let random_bounce_angle_change = std::f64::consts::FRAC_PI_2;

        return Parameters {
            parameter_file_path: env::current_dir()
                .expect("Current path is invalid")
                .to_str()
                .expect("to string failed on current path")
                .to_owned()
                + "\\src\\parameters.json",
            frame_period_ms: 0,
            display_scale: 5.,
            observer_parameters: ObserverParameters {
                look_vector_distance: (height / 2) as f64,
                look_up_angle: -look_angle,
                look_down_angle: look_angle,
                look_left_angle: -look_angle,
                look_right_angle: look_angle,
                min_hor_angle: 0.,
                max_hor_angle: 2. * std::f64::consts::PI,
                hor_angle_loop: true,
                min_ver_angle: -std::f64::consts::FRAC_PI_2,
                max_ver_angle: std::f64::consts::FRAC_PI_2,
                ver_angle_loop: false,
                move_forward_distance: move_distance,
                move_backward_distance: -move_distance,
                move_right_distance: move_distance,
                move_left_distance: -move_distance,
                move_up_distance: -move_distance,
                move_down_distance: move_distance,
                slow_mode_factor: 0.05,
                default_body: Sphere {
                    pos: Position {
                        x: 0.,
                        y: 0.,
                        z: 0.,
                    },
                    speed: Speed {
                        x: 0.,
                        y: 0.,
                        z: 0.,
                    },
                    radius: 1.,
                    color: Color {
                        r: 0,
                        g: 0,
                        b: 255,
                        a: 255,
                    },
                    light_factor: 100.,
                    type_: SphereType::Reflexive,
                    smoothness: 1.,
                    refractivity_index: 1.,
                    is_visible: true,
                },
            },
            ray_parameters: RayParameters {
                min_hor_value: -width / 2,
                max_hor_value: width / 2,
                min_ver_value: -height / 2,
                max_ver_value: height / 2,
                min_pixel_factor: 0.1,
                fog_factor: 0.,
                background_color: Color::RGB(0, 0, 0),
                background_light_factor: 1.,
                reflect_background: true,
                bounce_count: 5,
                bounce_color_reflection_factor: 1.,
                min_random_bounce_angle_change: -random_bounce_angle_change,
                max_random_bounce_angle_change: random_bounce_angle_change,
                reflect_inside_spheres: false,
            },
            sphere_parameters: vec![
                SphereParameters {
                    sphere_type: SphereType::Reflexive,
                    generation_mode: SphereGenerationMode::Random,
                    sphere_count: 10,
                    min_radius: 1.,
                    max_radius: 5.,
                    min_light_factor: 0.8,
                    max_light_factor: 1.,
                    min_smoothness: 0.,
                    max_smoothness: 1.,
                    min_refractivity_index: 1.,
                    max_refractivity_index: 1.,
                },
                SphereParameters {
                    sphere_type: SphereType::Refractive,
                    generation_mode: SphereGenerationMode::Random,
                    sphere_count: 10,
                    min_radius: 1.,
                    max_radius: 5.,
                    min_light_factor: 0.25,
                    max_light_factor: 1.,
                    min_smoothness: 1.,
                    max_smoothness: 1.,
                    min_refractivity_index: 1.05,
                    max_refractivity_index: 2.,
                },
            ],
            physics_parameters: PhysicsParameters {
                g: 0.002,
                enabled: false,
                min_x: -physics_bounds_value,
                max_x: physics_bounds_value,
                min_y: -physics_bounds_value,
                max_y: physics_bounds_value,
                min_z: -physics_bounds_value,
                max_z: physics_bounds_value,
                min_vx: -speed_bounds_value,
                max_vx: speed_bounds_value,
                min_vy: -speed_bounds_value,
                max_vy: speed_bounds_value,
                min_vz: -speed_bounds_value,
                max_vz: speed_bounds_value,
            },
        };
    }

    fn update_from_json(&mut self, data: &serde_json::Value) {
        self.frame_period_ms = *data["frame_period_ms"]
            .as_u64()
            .get_or_insert(self.frame_period_ms);
        self.display_scale = *data["display_scale"]
            .as_f64()
            .get_or_insert(self.display_scale);
        self.observer_parameters
            .update_from_json(&data["observer_parameters"]);
        self.ray_parameters
            .update_from_json(&data["ray_parameters"]);
        self.physics_parameters
            .update_from_json(&data["physics_parameters"]);
    }

    pub fn reload(&mut self) {
        println!("Reloading parameters...");
        let str: String = fs::read_to_string(self.parameter_file_path.to_string())
            .expect("Unable to read parameter file");

        let json: serde_json::Value =
            serde_json::from_str(&str).expect("JSON was not well-formatted");

        self.update_from_json(&json);

        println!("Parameters reloaded!");
    }
}
