pub enum Keys {
    W(bool),
    A(bool),
    S(bool),
    D(bool),
}



pub struct InputHandler {
    pressed: Vec<Keys>
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            pressed: vec![Keys::W(false), Keys::A(false), Keys::S(false), Keys::D(false)]
        }
    }
}