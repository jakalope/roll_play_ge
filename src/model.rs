
pub struct Model {
    pub x: u32,
    pub y: u32,
    pub vx: u32,
    pub vy: u32,
}

impl Model {
    pub fn new() -> Self {
        Model {
            x: 0,
            y: 0,
            vx: 0,
            vy: 0,
        }
    }
}
