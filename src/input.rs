
/// During event handling, if a button is reported as "pressed", that field in this struct is
/// set to `true`. If a button is reported as "released", that field is set to `false`.
#[derive(Debug)]
pub struct Input {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub button_a: bool,
    pub button_b: bool,
}

impl Input {
    pub fn new() -> Self {
        Input {
            up: false,
            down: false,
            left: false,
            right: false,
            button_a: false,
            button_b: false,
        }
    }
}
