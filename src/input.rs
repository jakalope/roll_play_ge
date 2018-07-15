use game_network::bitvec;

/// During event handling, if a button is reported as "pressed", that field in this struct is
/// set to `true`. If a button is reported as "released", that field is set to `false`.
#[derive(Debug)]
pub struct Input {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub attack: bool,
    pub defend: bool,
    pub menu: bool,
}

pub enum InputError {
    BadBitVec(usize),
}

pub enum Command {
    Up,
    Down,
    Left,
    Right,
    Attack,
    Defend,
    Menu,
}

impl Input {
    pub fn new() -> Self {
        Input {
            up: false,
            down: false,
            left: false,
            right: false,
            attack: false,
            defend: false,
            menu: false,
        }
    }

    pub fn set_command(&mut self, cmd: &Command, set: bool) {
        match cmd {
            Command::Up => self.up = set,
            Command::Down => self.down = set,
            Command::Left => self.left = set,
            Command::Right => self.right = set,
            Command::Attack => self.attack = set,
            Command::Defend => self.defend = set,
            Command::Menu => self.menu = set,
        }
    }

    pub fn from_bitvec(bv: bitvec::BitVec) -> Result<Self, InputError> {
        if bv.len() != 7 {
            return Err(InputError::BadBitVec(bv.len()));
        }
        Ok(Input {
            up: bv.get(0).unwrap(),
            down: bv.get(1).unwrap(),
            left: bv.get(2).unwrap(),
            right: bv.get(3).unwrap(),
            attack: bv.get(4).unwrap(),
            defend: bv.get(5).unwrap(),
            menu: bv.get(6).unwrap(),
        })
    }
}

impl<'a> From<&'a Input> for bitvec::BitVec {
    fn from(input: &Input) -> Self {
        let mut bv = bitvec::BitVec::new();
        bv.push(input.up);
        bv.push(input.down);
        bv.push(input.left);
        bv.push(input.right);
        bv.push(input.attack);
        bv.push(input.defend);
        bv.push(input.menu);
        bv
    }
}
