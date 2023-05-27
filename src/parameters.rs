use sdl2::pixels::Color;

use crate::position::Position;

pub struct Parameters {
    pub min_hor_ray_value: i32,
    pub max_hor_ray_value: i32,
    pub min_ver_ray_value: i32,
    pub max_ver_ray_value: i32,
    pub observer_look_vector_distance: f64,
    pub sphere_count: u32,
    pub g: f64,
    pub display_scale: f32,
    pub physics: bool,
    pub frame_period_ms: u64,
    pub observer_default_position: Position,
    pub observer_look_up_angle: f64,
    pub observer_look_down_angle: f64,
    pub observer_look_left_angle: f64,
    pub observer_look_right_angle: f64,
    pub observer_move_forward_distance: f64,
    pub observer_move_backward_distance: f64,
    pub observer_move_right_distance: f64,
    pub observer_move_left_distance: f64,
    pub observer_move_up_distance: f64,
    pub observer_move_down_distance: f64,
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
    pub min_sphere_radius: f64,
    pub max_sphere_radius: f64,
    pub min_pixel_factor: f64,
    pub fog_factor: f64,
    pub background_color: Color,
    pub ray_bounce_count: u32,
    pub ray_bounce_color_reflection_factor: f64,
}

impl Parameters {
    pub fn default() -> Parameters {
        let half_width: i32 = 200;
        let half_height: i32 = 100;
        let look_angle = 0.05;
        let move_distance = 0.25;
        let physics_bounds_value = 20.0;
        let speed_bounds_value = 0.25;

        return Parameters {
            min_hor_ray_value: -half_width,
            max_hor_ray_value: half_width,
            min_ver_ray_value: -half_width,
            max_ver_ray_value: half_height,
            observer_look_vector_distance: half_height as f64,
            sphere_count: 5,
            g: 0.001,
            display_scale: 2.5,
            physics: true,
            frame_period_ms: 0,
            observer_default_position: Position {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            observer_look_up_angle: -look_angle,
            observer_look_down_angle: look_angle,
            observer_look_left_angle: -look_angle,
            observer_look_right_angle: look_angle,
            observer_move_forward_distance: move_distance,
            observer_move_backward_distance: -move_distance,
            observer_move_right_distance: move_distance,
            observer_move_left_distance: -move_distance,
            observer_move_up_distance: -move_distance,
            observer_move_down_distance: move_distance,
            min_x: -physics_bounds_value,
            max_x: physics_bounds_value,
            min_y: -physics_bounds_value,
            max_y: physics_bounds_value,
            min_z: -physics_bounds_value,
            max_z: 10.0,
            min_vx: -speed_bounds_value,
            max_vx: speed_bounds_value,
            min_vy: -speed_bounds_value,
            max_vy: speed_bounds_value,
            min_vz: -speed_bounds_value,
            max_vz: speed_bounds_value,
            min_sphere_radius: 2.5,
            max_sphere_radius: 5.0,
            min_pixel_factor: 0.5,
            fog_factor: 5.0,
            background_color: Color::RGB(0, 0, 0),
            ray_bounce_count: 1,
            ray_bounce_color_reflection_factor: 0.5,
        };
    }
}
