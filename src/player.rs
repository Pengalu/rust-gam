use piston::UpdateArgs;

use crate::entity::Transform;

pub struct Player {
    _transform: Transform,
}


// im gonna cook u
impl Player {

    // struct for creating new player structs
    pub fn new(x: u64, y: u64) -> Player {
        // player struct with all of specified fields
        Player {
            _transform: Transform {
                x: x,
                y: y,
            },
        }
    }

    // update function for player struct which moves player
    pub fn update(&mut self, _args: &UpdateArgs) -> [f64; 2] {


        [6., 6.]
    }
}