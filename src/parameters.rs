use sdl2::pixels::Color;
use serde_json::Value;
use std::env;
use std::fs;

use crate::{
    position::Position,
    speed::Speed,
    sphere::{Sphere, SphereType},
};

fn get_parameter_file_path() -> String {
    return env::current_dir()
        .expect("Current path is invalid")
        .to_str()
        .expect("to string failed on current path")
        .to_owned()
        + "\\src\\parameters.json";
}

fn position_from_json(data: &Value, default: &Position) -> Position {
    return Position {
        x: *data["x"].as_f64().get_or_insert(default.x),
        y: *data["y"].as_f64().get_or_insert(default.y),
        z: *data["z"].as_f64().get_or_insert(default.z),
    };
}

fn speed_from_json(data: &Value, default: &Speed) -> Speed {
    return Speed {
        x: *data["x"].as_f64().get_or_insert(default.x),
        y: *data["y"].as_f64().get_or_insert(default.y),
        z: *data["z"].as_f64().get_or_insert(default.z),
    };
}

fn color_from_json(data: &Value, default: &Color) -> Color {
    return Color {
        r: *data["r"].as_u64().get_or_insert(default.r as u64) as u8,
        g: *data["g"].as_u64().get_or_insert(default.g as u64) as u8,
        b: *data["b"].as_u64().get_or_insert(default.b as u64) as u8,
        a: *data["a"].as_u64().get_or_insert(default.a as u64) as u8,
    };
}

fn type_from_json(data: &Value, default: &SphereType) -> SphereType {
    return SphereType::from_string(data.as_str().get_or_insert(default.to_string()));
}

fn sphere_from_json(data: &Value, default: &Sphere) -> Sphere {
    return Sphere {
        pos: position_from_json(&data["pos"], &default.pos),
        speed: speed_from_json(&data["speed"], &default.speed),
        radius: *data["radius"].as_f64().get_or_insert(default.radius),
        color: color_from_json(&data["color"], &default.color),
        light_factor: *data["light_factor"]
            .as_f64()
            .get_or_insert(default.light_factor),
        type_: type_from_json(&data["type_"], &default.type_),
        smoothness: *data["smoothness"]
            .as_f64()
            .get_or_insert(default.smoothness),
        refractivity_index: *data["refractivity_index"]
            .as_f64()
            .get_or_insert(default.refractivity_index),
        is_visible: *data["is_visible"]
            .as_bool()
            .get_or_insert(default.is_visible),
    };
}

fn generation_mode_from_json(data: &Value, default: &SphereGenerationMode) -> SphereGenerationMode {
    return SphereGenerationMode::from_string(data.as_str().get_or_insert(default.to_string()));
}

pub enum SphereGenerationMode {
    Hardcoded,
    Random,
    InLine,
}

impl SphereGenerationMode {
    pub fn to_string(&self) -> &str {
        return match *self {
            Self::Hardcoded => "Hardcoded",
            Self::Random => "Random",
            Self::InLine => "InLine",
        };
    }

    pub fn from_string(string: &str) -> Self {
        return match string {
            "Reflexive" => Self::Hardcoded,
            "Random" => Self::Random,
            "InLine" => Self::InLine,
            _ => Self::Hardcoded,
        };
    }
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
    fn get_from_json(data: &Value) -> Self {
        return ObserverParameters {
            look_vector_distance: *data["look_vector_distance"].as_f64().get_or_insert(64.),
            look_up_angle: *data["look_up_angle"].as_f64().get_or_insert(-0.1),
            look_down_angle: *data["look_down_angle"].as_f64().get_or_insert(0.1),
            look_left_angle: *data["look_left_angle"].as_f64().get_or_insert(-0.1),
            look_right_angle: *data["look_right_angle"].as_f64().get_or_insert(0.1),
            min_hor_angle: *data["min_hor_angle"].as_f64().get_or_insert(0.),
            max_hor_angle: *data["max_hor_angle"]
                .as_f64()
                .get_or_insert(2. * std::f64::consts::PI),
            hor_angle_loop: *data["hor_angle_loop"].as_bool().get_or_insert(true),
            min_ver_angle: *data["min_ver_angle"]
                .as_f64()
                .get_or_insert(-std::f64::consts::FRAC_PI_2),
            max_ver_angle: *data["max_ver_angle"]
                .as_f64()
                .get_or_insert(std::f64::consts::FRAC_PI_2),
            ver_angle_loop: *data["ver_angle_loop"].as_bool().get_or_insert(false),
            move_forward_distance: *data["move_forward_distance"].as_f64().get_or_insert(0.5),
            move_backward_distance: *data["move_backward_distance"].as_f64().get_or_insert(-0.5),
            move_right_distance: *data["move_right_distance"].as_f64().get_or_insert(0.5),
            move_left_distance: *data["move_left_distance"].as_f64().get_or_insert(-0.5),
            move_up_distance: *data["move_up_distance"].as_f64().get_or_insert(0.5),
            move_down_distance: *data["move_down_distance"].as_f64().get_or_insert(-0.5),
            slow_mode_factor: *data["slow_mode_factor"].as_f64().get_or_insert(0.05),
            default_body: sphere_from_json(
                &data["default_body"],
                &Sphere {
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
            ),
        };
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
    fn get_from_json(data: &Value) -> Self {
        return RayParameters {
            min_hor_value: *data["min_hor_value"].as_i64().get_or_insert(-64),
            max_hor_value: *data["max_hor_value"].as_i64().get_or_insert(64),
            min_ver_value: *data["min_ver_value"].as_i64().get_or_insert(-64),
            max_ver_value: *data["max_ver_value"].as_i64().get_or_insert(64),
            min_pixel_factor: *data["min_pixel_factor"].as_f64().get_or_insert(0.1),
            fog_factor: *data["fog_factor"].as_f64().get_or_insert(0.1),
            background_color: color_from_json(&data["background_color"], &Color::RGB(0, 0, 0)),
            background_light_factor: *data["background_light_factor"].as_f64().get_or_insert(1.),
            reflect_background: *data["reflect_background"].as_bool().get_or_insert(true),
            bounce_count: *data["bounce_count"].as_u64().get_or_insert(5),
            bounce_color_reflection_factor: *data["bounce_color_reflection_factor"]
                .as_f64()
                .get_or_insert(1.),
            min_random_bounce_angle_change: *data["min_random_bounce_angle_change"]
                .as_f64()
                .get_or_insert(-std::f64::consts::FRAC_PI_2),
            max_random_bounce_angle_change: *data["max_random_bounce_angle_change"]
                .as_f64()
                .get_or_insert(std::f64::consts::FRAC_PI_2),
            reflect_inside_spheres: *data["reflect_inside_spheres"]
                .as_bool()
                .get_or_insert(false),
        };
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

impl SphereParameters {
    fn get_vec_from_json(data: &Value, default: &Self) -> Vec<Self> {
        let value_array_opt: Option<&Vec<Value>> = data.as_array();
        return match value_array_opt {
            Some(value_array) => value_array
                .iter()
                .map(|value: &Value| {
                    return SphereParameters::get_from_json(value, default);
                })
                .collect(),
            None => vec![],
        };
    }

    fn get_from_json(data: &Value, default: &Self) -> Self {
        return SphereParameters {
            generation_mode: generation_mode_from_json(
                &data["generation_mode"],
                &default.generation_mode,
            ),
            sphere_type: type_from_json(&data["sphere_type"], &default.sphere_type),
            sphere_count: *data["sphere_count"]
                .as_u64()
                .get_or_insert(default.sphere_count),
            min_radius: *data["min_radius"]
                .as_f64()
                .get_or_insert(default.min_radius),
            max_radius: *data["max_radius"]
                .as_f64()
                .get_or_insert(default.max_radius),
            min_light_factor: *data["min_light_factor"]
                .as_f64()
                .get_or_insert(default.min_light_factor),
            max_light_factor: *data["max_light_factor"]
                .as_f64()
                .get_or_insert(default.max_light_factor),
            min_smoothness: *data["min_smoothness"]
                .as_f64()
                .get_or_insert(default.min_smoothness),
            max_smoothness: *data["max_smoothness"]
                .as_f64()
                .get_or_insert(default.max_smoothness),
            min_refractivity_index: *data["min_refractivity_index"]
                .as_f64()
                .get_or_insert(default.min_refractivity_index),
            max_refractivity_index: *data["max_refractivity_index"]
                .as_f64()
                .get_or_insert(default.max_refractivity_index),
        };
    }
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
    fn get_from_json(data: &Value) -> Self {
        return PhysicsParameters {
            g: *data["g"].as_f64().get_or_insert(0.002),
            enabled: *data["enabled"].as_bool().get_or_insert(false),
            min_x: *data["min_x"].as_f64().get_or_insert(-20.),
            max_x: *data["max_x"].as_f64().get_or_insert(20.),
            min_y: *data["min_y"].as_f64().get_or_insert(-20.),
            max_y: *data["max_y"].as_f64().get_or_insert(20.),
            min_z: *data["min_z"].as_f64().get_or_insert(-20.),
            max_z: *data["max_z"].as_f64().get_or_insert(20.),
            min_vx: *data["min_vx"].as_f64().get_or_insert(-0.0025),
            max_vx: *data["max_vx"].as_f64().get_or_insert(0.0025),
            min_vy: *data["min_vy"].as_f64().get_or_insert(-0.0025),
            max_vy: *data["max_vy"].as_f64().get_or_insert(0.0025),
            min_vz: *data["min_vz"].as_f64().get_or_insert(-0.0025),
            max_vz: *data["max_vz"].as_f64().get_or_insert(0.0025),
        };
    }
}

pub struct Parameters {
    pub frame_period_ms: u64,
    pub display_scale: f64,
    pub observer_parameters: ObserverParameters,
    pub ray_parameters: RayParameters,
    pub sphere_parameters: Vec<SphereParameters>,
    pub physics_parameters: PhysicsParameters,
}

impl Parameters {
    pub fn get_from_json() -> Self {
        println!("Reading parameters file...");
        let str: String = fs::read_to_string(get_parameter_file_path().to_string())
            .expect("Unable to read parameter file");

        let data: Value = serde_json::from_str(&str).expect("JSON was not well-formatted");

        return Parameters {
            frame_period_ms: *data["frame_period_ms"].as_u64().get_or_insert(0),
            display_scale: *data["display_scale"].as_f64().get_or_insert(5.),
            observer_parameters: ObserverParameters::get_from_json(&data["observer_parameters"]),
            ray_parameters: RayParameters::get_from_json(&data["ray_parameters"]),
            sphere_parameters: SphereParameters::get_vec_from_json(
                &data["sphere_parameters"],
                &SphereParameters {
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
            ),
            physics_parameters: PhysicsParameters::get_from_json(&data["physics_parameters"]),
        };
    }
}
