pub struct Parameters {
    pub w: u32,
    pub h: u32,
    pub sphere_count: u32,
    pub g: f64,
    
}

impl Parameters {
    pub fn default() -> Parameters {
        return Parameters {
            w: 100,
            h: 100,
            sphere_count: 3,
            g: 0.001,
        };
    }
}
