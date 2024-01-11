use sdl2::pixels::Color;

use crate::position::Position;

pub struct ObserverParameters {
    pub look_vector_distance: f64,
    pub default_position: Position,
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
}

pub struct RayParameters {
    pub min_hor_value: i32,
    pub max_hor_value: i32,
    pub min_ver_value: i32,
    pub max_ver_value: i32,
    pub min_pixel_factor: f64,
    pub fog_factor: f64,
    pub background_color: Color,
    pub bounce_count: u32,
    pub bounce_color_reflection_factor: f64,
    pub min_random_bounce_angle_change: f64,
    pub max_random_bounce_angle_change: f64,
}

pub struct SphereParameters {
    pub sphere_count: u32,
    pub min_radius: f64,
    pub max_radius: f64,
    pub min_light_factor: f64,
    pub max_light_factor: f64,
    pub min_reflexivity_factor: f64,
    pub max_reflexivity_factor: f64,
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
    pub sphere_parameters: SphereParameters,
    pub physics_parameters: PhysicsParameters,
}

impl Parameters {
    pub fn default() -> Parameters {
        let width: i32 = 640;
        let height: i32 = 320;
        let look_angle = 0.05;
        let move_distance = 0.25;
        let physics_bounds_value = 20.0;
        let speed_bounds_value = 0.01;

        return Parameters {
            frame_period_ms: 1,
            display_scale: 2.,
            observer_parameters: ObserverParameters {
                look_vector_distance: (height / 2) as f64,
                default_position: Position {
                    x: -physics_bounds_value,
                    y: 0.0,
                    z: 0.0,
                },
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
            },
            ray_parameters: RayParameters {
                min_hor_value: -width / 2,
                max_hor_value: width / 2,
                min_ver_value: -height / 2,
                max_ver_value: height / 2,
                min_pixel_factor: 0.,
                fog_factor: 0.25,
                background_color: Color::RGB(0, 0, 0),
                bounce_count: 6,
                bounce_color_reflection_factor: 0.75,
                min_random_bounce_angle_change: 0.,
                max_random_bounce_angle_change: std::f64::consts::FRAC_PI_2,
            },
            sphere_parameters: SphereParameters {
                sphere_count: 100,
                min_radius: 0.5,
                max_radius: 5.0,
                min_light_factor: 0.,
                max_light_factor: 10.,
                min_reflexivity_factor: 0.,
                max_reflexivity_factor: 1.,
            },
            physics_parameters: PhysicsParameters {
                g: 0.00001,
                enabled: true,
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
