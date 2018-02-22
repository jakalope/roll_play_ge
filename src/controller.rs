use input;

#[derive(Debug)]
pub struct Controller {
    pub input: input::Input,
}

impl Controller {
    pub fn new() -> Self {
        Controller { input: input::Input::new() }
    }
}
