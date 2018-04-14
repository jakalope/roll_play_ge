use input;

#[derive(Debug)]
pub struct Controller {
    /// Controller input, e.g. walking direction.
    pub input: input::Input,

    /// Walk-Rate multiplier. Normal ~= 90.0.
    pub walk_rate: f32,

    /// Time since last game cycle.
    pub dt_s: f64,

    /// Total elapsed game time.
    pub game_time_s: f64,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            input: input::Input::new(),
            walk_rate: 90.0,
            dt_s: 0.0,
            game_time_s: 0.0,
        }
    }
}
