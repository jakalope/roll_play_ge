use input;

#[derive(Debug)]
pub struct Controller {
    pub input: input::Input,
    pub walk_rate: f32,
    pub dt_s: f64,
    pub game_time_s: f64,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            input: input::Input::new(),
            walk_rate: 240.0,
            dt_s: 0.0,
            game_time_s: 0.0,
        }
    }
}
