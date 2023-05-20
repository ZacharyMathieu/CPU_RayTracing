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
    pub observer_look_up_angle: f64,
    pub observer_look_down_angle: f64,
    pub observer_look_left_angle: f64,
    pub observer_look_right_angle: f64,
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
    pub min_z: f64,
    pub max_z: f64,
    pub min_pixel_factor: f64,
    pub fog_factor: f64,
}

impl Parameters {
    pub fn default() -> Parameters {
        let half_width: i32 = 150;
        let half_height: i32 = 75;
        let physics_bounds_value = 25.0;

        return Parameters {
            min_hor_ray_value: -half_width,
            max_hor_ray_value: half_width,
            min_ver_ray_value: -half_height,
            max_ver_ray_value: half_height,
            observer_look_vector_distance: 150.0,
            sphere_count: 50,
            g: 0.0,
            display_scale: 4.0,
            physics: true,
            frame_period_ms: 0,
            observer_look_up_angle: -0.1,
            observer_look_down_angle: 0.1,
            observer_look_left_angle: -0.1,
            observer_look_right_angle: 0.1,
            min_x: 0.0,
            max_x: physics_bounds_value,
            min_y: -physics_bounds_value,
            max_y: physics_bounds_value,
            min_z: -physics_bounds_value,
            max_z: physics_bounds_value,
            min_pixel_factor: 0.5,
            fog_factor: 5.0,
        };
    }
}
