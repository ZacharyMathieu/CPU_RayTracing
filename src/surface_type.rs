
#[derive(Clone, Copy)]
pub enum SurfaceType {
    Reflexive,
    Refractive,
}

impl SurfaceType {
    pub fn to_string(&self) -> &str {
        return match *self {
            Self::Reflexive => "Reflexive",
            Self::Refractive => "Refractive",
        };
    }

    pub fn from_string(string: &str) -> Self {
        return match string {
            "Reflexive" => Self::Reflexive,
            "Refractive" => Self::Refractive,
            _ => Self::Reflexive,
        };
    }
}