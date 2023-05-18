pub struct Parameters {
    pub w: u32,
    pub h: u32,
    pub half_w: i32,
    pub half_h: i32,
    pub sphere_count: u32,
    pub g: f64,
    pub display_scale: f32,
    pub physics: bool,
    pub frame_period_ms: u64,
}

impl Parameters {
    pub fn default() -> Parameters {
        let w: u32 = 25;
        let h: u32 = 25;
        return Parameters {
            w: w,
            h: h,
            half_w: w as i32 / 2,
            half_h: h as i32 / 2,
            sphere_count: 1,
            g: 0.0,
            display_scale: 20.0,
            physics: true,
            frame_period_ms: 10,
        };
    }
}
