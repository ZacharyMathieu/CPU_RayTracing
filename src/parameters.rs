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
}

impl Parameters {
    // Horrible way to make a factory
    pub fn default() -> Parameters {
        return Parameters {
            min_hor_ray_value: -50,
            max_hor_ray_value: 50,
            min_ver_ray_value: -50,
            max_ver_ray_value: 50,
            observer_look_vector_distance: 100.0,
            sphere_count: 5,
            g: 0.0,
            display_scale: 5.0,
            physics: true,
            frame_period_ms: 0,
            observer_look_up_angle: -0.1,
            observer_look_down_angle: 0.1,
            observer_look_left_angle: -0.1,
            observer_look_right_angle: 0.1,
            min_x: -100.0,
            max_x: 100.0,
            min_y: -100.0,
            max_y: 100.0,
            min_z: -100.0,
            max_z: 100.0,
        };
    }
}
