use piston::{UpdateArgs, ButtonArgs};

pub struct Player {
    pub transform: (f64, f64),
}


// im gonna cook u
impl Player {
    // struct for creating new player structs
    pub fn new(x: f64, y: f64) -> Player {
        // player struct with all of specified fields
        Player {
            transform: (x, y)
        }
    }
    // update function for player struct which moves player
    pub fn update(&mut self, args: &UpdateArgs) -> [f64; 2] {
        self.transform.0 += 800. * args.dt;
        self.transform.1 += 800. * args.dt;

        [6., 6.]
    }
    pub fn update_keys(&mut self, args: &ButtonArgs) {

    }
}