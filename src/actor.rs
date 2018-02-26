use piston_window::{G2d, G2dTexture};
use graphics::math::Matrix2d;
use sprite;
use std;
use std::cell;
use gfx_texture::ImageSize;
use std::collections::HashMap;

pub struct CharacterSheet {
    texture: std::rc::Rc<G2dTexture>,
    tiles_wide: u32,
    tiles_high: u32,
    tile_width: u32,
    tile_height: u32,
}

impl CharacterSheet {
    pub fn tiles_wide(&self) -> u32 {
        self.tiles_wide
    }
    pub fn tiles_high(&self) -> u32 {
        self.tiles_high
    }
    pub fn tile_width(&self) -> u32 {
        self.tile_width
    }
    pub fn tile_height(&self) -> u32 {
        self.tile_height
    }

    /// Create a character sheet from a texture.
    ///
    /// `texture`: A texture containing a full character sheet.
    /// `tiles_wide`: Number of tile columns in `texture`.
    /// `tiles_high`: Number of tile rows in `texture`.
    ///
    /// Total tile count = `tiles_wide * tiles_high`.
    /// Returns an error if `texture.get_width() % tiles_wide !=0` or
    ///   `texture.get_height() % tiles_high !=0`.
    /// Otherwise, returns the newly created `CharacterSheet`.
    pub fn from_texture(
        texture: G2dTexture,
        tiles_wide: u32,
        tiles_high: u32,
    ) -> Result<Self, String> {
        if texture.get_width() % tiles_wide != 0 {
            return Err(String::from("texture width not divisible by tiles_wide"));
        }
        if texture.get_height() % tiles_high != 0 {
            return Err(String::from("texture height not divisible by tiles_high"));
        }
        let tile_width = texture.get_width() / tiles_wide;
        let tile_height = texture.get_height() / tiles_high;
        Ok(CharacterSheet {
            texture: std::rc::Rc::<G2dTexture>::new(texture),
            tiles_wide: tiles_wide,
            tiles_high: tiles_high,
            tile_width: tile_width,
            tile_height: tile_height,
        })
    }

    pub fn sprite(
        &self,
        tile_column: u32,
        tile_row: u32,
    ) -> Result<sprite::Sprite<G2dTexture>, String> {
        if tile_column >= self.tiles_wide() {
            return Err(String::from("tile_column must be less than tiles_wide()"));
        }
        if tile_row >= self.tiles_high() {
            return Err(String::from("tile_row must be less than tiles_high()"));
        }

        let src_rect = [
            (self.tile_width() * tile_column) as f64,
            (self.tile_height() * tile_row) as f64,
            self.tile_width() as f64,
            self.tile_height() as f64,
        ];

        Ok(sprite::Sprite::<G2dTexture>::from_texture_rect(
            self.texture.clone(),
            src_rect,
        ))
    }
}

pub struct ChooserArgs {
    pub vx: f32,
    pub vy: f32,
    pub game_time_s: f64,
}

impl ChooserArgs {
    pub fn new() -> Self {
        ChooserArgs {
            vx: 0.0,
            vy: 0.0,
            game_time_s: 0.0,
        }
    }

    pub fn from_vals(vx: f32, vy: f32, game_time_s: f64) -> Self {
        ChooserArgs {
            vx: vx,
            vy: vy,
            game_time_s: game_time_s,
        }
    }
}

pub trait SpriteChooser {
    fn choose(&mut self, args: ChooserArgs) -> Option<&sprite::Sprite<G2dTexture>>;
}

fn choose_walking_column(len: usize, dt: f64, args: &ChooserArgs) -> u32 {
    if args.vx == 0.0 && args.vy == 0.0 {
        1 // Stationary
    } else {
        // TODO: the preferred sequence is 0, 1, 2, 1, 0, ...
        //       the current sequence is 0, 1, 2, 0, 1, 2, ...
        let modulo = args.game_time_s % (len as f64 * dt);
        let factor = modulo / dt;
        factor as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::choose_walking_column as choose;

    #[test]
    fn stationary_column() {
        assert_eq!(1, choose(2, 1.0, &ChooserArgs::from_vals(0.0, 0.0, 0.0)));
        assert_eq!(1, choose(2, 1.0, &ChooserArgs::from_vals(0.0, 0.0, 2.2)));
    }

    #[test]
    fn walking_column() {
        assert_eq!(0, choose(2, 1.0, &ChooserArgs::from_vals(1.0, 0.0, 0.0)));
        assert_eq!(1, choose(2, 1.0, &ChooserArgs::from_vals(1.0, 0.0, 1.1)));
        assert_eq!(0, choose(2, 1.0, &ChooserArgs::from_vals(1.0, 0.0, 2.2)));
        assert_eq!(1, choose(2, 1.0, &ChooserArgs::from_vals(1.0, 0.0, 3.3)));

        assert_eq!(0, choose(3, 1.0, &ChooserArgs::from_vals(1.0, 0.0, 0.0)));
        assert_eq!(1, choose(3, 1.0, &ChooserArgs::from_vals(1.0, 0.0, 1.1)));
        assert_eq!(2, choose(3, 1.0, &ChooserArgs::from_vals(1.0, 0.0, 2.2)));
        assert_eq!(0, choose(3, 1.0, &ChooserArgs::from_vals(1.0, 0.0, 3.3)));
        assert_eq!(1, choose(3, 1.0, &ChooserArgs::from_vals(1.0, 0.0, 4.4)));
        assert_eq!(2, choose(3, 1.0, &ChooserArgs::from_vals(1.0, 0.0, 5.5)));
        assert_eq!(0, choose(3, 1.0, &ChooserArgs::from_vals(1.0, 0.0, 6.0)));
        assert_eq!(0, choose(3, 1.0, &ChooserArgs::from_vals(1.0, 0.0, 6.6)));

        assert_eq!(0, choose(2, 0.5, &ChooserArgs::from_vals(1.0, 0.0, 0.0)));
        assert_eq!(1, choose(2, 0.5, &ChooserArgs::from_vals(1.0, 0.0, 0.6)));
        assert_eq!(0, choose(2, 0.5, &ChooserArgs::from_vals(1.0, 0.0, 1.1)));
        assert_eq!(1, choose(2, 0.5, &ChooserArgs::from_vals(1.0, 0.0, 1.7)));
    }
}

fn choose_walking_row(previous_row: u32, args: &ChooserArgs) -> u32 {
    if args.vx > 0.0 {
        // Right
        1
    } else if args.vx < 0.0 {
        // Left
        2
    } else if args.vy > 0.0 {
        // Down
        3
    } else if args.vy < 0.0 {
        // Up
        0
    } else {
        previous_row
    }
}

pub struct WalkingSpriteChooser {
    sprite: Vec<sprite::Sprite<G2dTexture>>,
    dt: f64, // time between sprite transitions
    previous_row: u32, // previous row used from the sprite sheet
}

impl WalkingSpriteChooser {
    pub fn from_sheet(dt: f64, sheet: &CharacterSheet) -> Result<Self, String> {
        if sheet.tiles_high() != 4 || sheet.tiles_wide() != 3 {
            return Err(String::from(
                "WalkingSpriteChooser::from_sheet() requires a 3x4 tile CharacterSheet",
            ));
        }

        let mut sprite_list = Vec::<sprite::Sprite<G2dTexture>>::new();
        for row in 0..4 {
            for col in 0..3 {
                let sprite = sheet.sprite(col, row)?;
                sprite_list.push(sprite);
            }
        }

        Ok(WalkingSpriteChooser {
            sprite: sprite_list,
            dt: dt,
            previous_row: 0,
        })
    }
}

impl SpriteChooser for WalkingSpriteChooser {
    fn choose(&mut self, args: ChooserArgs) -> Option<&sprite::Sprite<G2dTexture>> {
        match self.sprite.len() {
            0 => {
                return None;
            }
            len => Some({
                let mut column = choose_walking_column(3, self.dt, &args);
                self.previous_row = choose_walking_row(self.previous_row, &args);
                let idx = (self.previous_row * 3 + column) as usize;
                assert!(idx < 12);
                &self.sprite[idx]
            }),
        }
    }
}

pub enum ActorDrawError {
    NothingToDraw,
    NoSuchName,
}

pub struct Actor {
    pub x: f32, // in px
    pub y: f32, // in px
    pub vx: f32, // in px per frame
    pub vy: f32, // in px per frame
    chooser_map: HashMap<String, cell::RefCell<Box<SpriteChooser>>>,
}

impl Actor {
    /// Insert a named `SpriteChooser`. The given `name` is used as a handle to `draw()`.
    /// If `name` already existed in the chooser map, the previous chooser is returned.
    /// Otherwise returns `None`.
    pub fn insert_chooser(
        &mut self,
        name: String,
        chooser: cell::RefCell<Box<SpriteChooser>>,
    ) -> Option<cell::RefCell<Box<SpriteChooser>>> {
        self.chooser_map.insert(name, chooser)
    }

    pub fn new() -> Self {
        Actor {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            chooser_map: HashMap::<String, cell::RefCell<Box<SpriteChooser>>>::new(),
        }
    }

    pub fn draw(
        &mut self,
        name: &str,
        game_time_s: f64,
        transform: Matrix2d,
        renderer: &mut G2d,
    ) -> Result<(), ActorDrawError> {
        // Draw the correct frame from the correct animation.
        match self.chooser_map.get_mut(name) {
            Some(chooser) => {
                let mut borrow = chooser.borrow_mut();
                let choice = borrow.choose(ChooserArgs::from_vals(self.vx, self.vy, game_time_s));
                match choice {
                    Some(sprite) => {
                        sprite.draw(transform, renderer);
                    }
                    None => {
                        return Err(ActorDrawError::NothingToDraw);
                    }
                }

                return Ok(());
            }
            None => {
                return Err(ActorDrawError::NoSuchName);
            }
        }
    }
}
