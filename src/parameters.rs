use sdl2::pixels::Color;

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

pub struct RayParameters {
    pub min_hor_value: i32,
    pub max_hor_value: i32,
    pub min_ver_value: i32,
    pub max_ver_value: i32,
    pub min_pixel_factor: f64,
    pub fog_factor: f64,
    pub background_color: Color,
    pub background_light_factor: f64,
    pub reflect_background: bool,
    pub bounce_count: u32,
    pub bounce_color_reflection_factor: f64,
    pub min_random_bounce_angle_change: f64,
    pub max_random_bounce_angle_change: f64,
    pub reflect_inside_spheres: bool,
}

pub struct SphereParameters {
    pub generation_mode: SphereGenerationMode,
    pub sphere_type: SphereType,
    pub sphere_count: u32,
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

pub struct Parameters {
    pub frame_period_ms: u64,
    pub display_scale: f32,
    pub observer_parameters: ObserverParameters,
    pub ray_parameters: RayParameters,
    pub sphere_parameters: Vec<SphereParameters>,
    pub physics_parameters: PhysicsParameters,
}

impl Parameters {
    pub fn default() -> Parameters {
        let width: i32 = 256;
        let height: i32 = 128;
        let look_angle = 0.1;
        let move_distance = 0.5;
        let physics_bounds_value = 20.;
        let speed_bounds_value = 0.0025;
        let random_bounce_angle_change = std::f64::consts::FRAC_PI_2;

        return Parameters {
            frame_period_ms: 0,
            display_scale: 5.5,
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
                        b: 0,
                        a: 255,
                    },
                    light_factor: 10.,
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
                background_color: Color::RGB(255, 255, 255),
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
                    sphere_count: 20,
                    min_radius: 0.5,
                    max_radius: 10.,
                    min_light_factor: 0.5,
                    max_light_factor: 1.,
                    min_smoothness: 0.5,
                    max_smoothness: 0.8,
                    min_refractivity_index: 1.,
                    max_refractivity_index: 1.,
                },
                SphereParameters {
                    sphere_type: SphereType::Refractive,
                    generation_mode: SphereGenerationMode::Random,
                    sphere_count: 20,
                    min_radius: 0.5,
                    max_radius: 10.,
                    min_light_factor: 0.,
                    max_light_factor: 0.5,
                    min_smoothness: 0.995,
                    max_smoothness: 1.,
                    min_refractivity_index: 1.,
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
}
