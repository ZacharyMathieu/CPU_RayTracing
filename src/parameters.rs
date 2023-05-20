pub struct Parameters {
    pub min_hor_angle: f64,
    pub max_hor_angle: f64,
    pub hor_rays: u32,
    pub min_ver_angle: f64,
    pub max_ver_angle: f64,
    pub ver_rays: u32,
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
            min_hor_angle: -2.0,
            max_hor_angle: 2.0,
            hor_rays: 100,
            min_ver_angle: -2.0,
            max_ver_angle: 2.0,
            ver_rays: 100,
            observer_look_vector_distance: 1.0,
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
