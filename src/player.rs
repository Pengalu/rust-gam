use crate::entity::Transform;

pub struct Player {
    transform: Transform,
}

impl Player {
    pub fn new(x: u64, y: u64) -> Player {
        Player {
            transform: Transform {
                x: x,
                y: y,
            },
        }
    }
}