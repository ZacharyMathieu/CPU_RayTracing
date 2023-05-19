pub struct Parameters {
    pub w_rays: u32,
    pub h_rays: u32,
    pub observer_look_vector_distance: f64,
    pub sphere_count: u32,
    pub g: f64,
    pub display_scale: f32,
    pub physics: bool,
    pub frame_period_ms: u64,
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
            w_rays: 200,
            h_rays: 200,
            observer_look_vector_distance: 20.0,
            sphere_count: 10,
            g: 0.25,
            display_scale: 4.0,
            physics: true,
            frame_period_ms: 0,
            min_x: -100.0,
            max_x: 100.0,
            min_y: -100.0,
            max_y: 100.0,
            min_z: -100.0,
            max_z: 100.0,
        };
    }
}
