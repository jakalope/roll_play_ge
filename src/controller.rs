use input;

#[derive(Debug)]
pub struct Controller {
    pub input: input::Input,
    pub dt_s: f32,
    pub walk_rate: f32,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            input: input::Input::new(),
            dt_s: 0.0,
            walk_rate: 240.0,
        }
    }
}
