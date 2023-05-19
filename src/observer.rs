use crate::{parameters::Parameters, position::Position};

pub struct Observer {
    pub pos: Position,
    pub direction: Position,
}

impl Observer {
    pub fn default(parameters: &Parameters) -> Observer {
        return Observer {
            pos: Position {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Position {
                x: parameters.observer_look_vector_distance,
                y: 0.0,
                z: 0.0,
            },
        };
    }
}
