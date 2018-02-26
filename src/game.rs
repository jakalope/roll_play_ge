use std::*;
use piston_window;
use gfx_core;
use tilesheet;
use piston_window::*;
use controller;
use actor;

#[derive(Debug)]
pub enum NewGameError {
    TilesheetError(tilesheet::TilesheetError),
    WindowError(gfx_core::factory::CombinedError),
    HeroError(String),
}

pub struct Game {
    tilesheet: tilesheet::Tilesheet,
    piston_image: piston_window::Image,
    map_tiles_texture: piston_window::G2dTexture,
    controller: controller::Controller,
    hero: actor::Actor,
}

impl Game {
    pub fn from_path(
        asset_path: &path::Path,
        window: &mut piston_window::PistonWindow,
    ) -> Result<Self, NewGameError> {
        let tilesheet = tilesheet::Tilesheet::from_path(&asset_path.join("tiled_base64_zlib.tmx"))
            .map_err(|e| NewGameError::TilesheetError(e))?;


        let texture_settings = piston_window::TextureSettings::new();
        let map_tiles_texture = piston_window::Texture::from_image(
            &mut window.factory,
            tilesheet.image(),
            &texture_settings,
        ).map_err(|e| NewGameError::WindowError(e))?;

        let hero_texture = piston_window::Texture::from_path(
            &mut window.factory,
            &asset_path.join("hero_walk.png"),
            piston_window::Flip::None,
            &texture_settings,
        ).map_err(|e| NewGameError::HeroError(e))?;

        let hero_sheet = actor::CharacterSheet::from_texture(hero_texture, 3, 4)
            .map_err(|e| NewGameError::HeroError(e))?;

        let hero_walk_chooer = actor::WalkingSpriteChooser::from_sheet(0.2, &hero_sheet)
            .map_err(|e| NewGameError::HeroError(e))?;

        let mut hero = actor::Actor::new();
        hero.insert_chooser(
            String::from("walk"),
            cell::RefCell::<Box<actor::WalkingSpriteChooser>>::new(
                Box::<actor::WalkingSpriteChooser>::new(hero_walk_chooer),
            ),
        );

        Ok(Game {
            tilesheet: tilesheet,
            piston_image: piston_window::Image::new(),
            map_tiles_texture: map_tiles_texture,
            controller: controller::Controller::new(),
            hero: hero,
        })
    }

    pub fn next(&mut self, window: &mut piston_window::PistonWindow) -> bool {
        let event = match window.next() {
            Some(e) => e,
            None => {
                return false;
            }
        };
        self.update_controller(&event);

        self.update_actor();

        // Update view
        window.draw_2d(&event, |context, gfx| {
            self.render(context, gfx);
            Some(())
        });
        true
    }

    fn update_controller(&mut self, event: &piston_window::Event) {
        self.controller.dt_s = match event.idle_args() {
            Some(_args) => _args.dt,
            None => 0.0,
        };

        self.controller.dt_s = match event.update_args() {
            Some(_args) => _args.dt,
            None => 0.0,
        };

        self.controller.game_time_s += self.controller.dt_s;

        if let Some(Button::Keyboard(key)) = event.press_args() {
            // TODO Make keybindings configurable.
            match key {
                keyboard::Key::W => {
                    self.controller.input.up = true;
                }
                keyboard::Key::A => {
                    self.controller.input.left = true;
                }
                keyboard::Key::S => {
                    self.controller.input.down = true;
                }
                keyboard::Key::D => {
                    self.controller.input.right = true;
                }
                keyboard::Key::J => {
                    self.controller.input.button_a = true;
                }
                keyboard::Key::K => {
                    self.controller.input.button_b = true;
                }
                _ => {}
            }
        };
        if let Some(Button::Keyboard(key)) = event.release_args() {
            match key {
                keyboard::Key::W => {
                    self.controller.input.up = false;
                }
                keyboard::Key::A => {
                    self.controller.input.left = false;
                }
                keyboard::Key::S => {
                    self.controller.input.down = false;
                }
                keyboard::Key::D => {
                    self.controller.input.right = false;
                }
                keyboard::Key::J => {
                    self.controller.input.button_a = false;
                }
                keyboard::Key::K => {
                    self.controller.input.button_b = false;
                }
                _ => {}
            }
        };
        if let Some(touch) = event.touch_args() {
            // http://docs.piston.rs/piston_window/input/struct.TouchArgs.html
            // TODO When on a mobile platform, display touch interface and
            // use this event to register "button" presses.
            println!("Touch occurred '{:?}'", touch);
        };
    }

    fn update_actor(&mut self) {
        self.hero.vy = self.controller.walk_rate *
            ((self.controller.input.up as i32) - (self.controller.input.down as i32)) as f32;
        self.hero.vx = self.controller.walk_rate *
            ((self.controller.input.left as i32) - (self.controller.input.right as i32)) as f32;
        self.hero.y += (self.controller.dt_s * self.hero.vy as f64) as f32;
        self.hero.x += (self.controller.dt_s * self.hero.vx as f64) as f32;
    }

    fn render(&mut self, context: piston_window::Context, renderer: &mut G2d) {
        piston_window::clear([0.5; 4], renderer);

        for (y, row) in self.tilesheet.layer_tile_iter(0).enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                if tile == 0 {
                    continue;
                }

                let trans = context.transform.trans(
                    self.hero.x as f64 -
                        x as f64 * self.tilesheet.tile_width() as f64,
                    self.hero.y as f64 -
                        y as f64 * self.tilesheet.tile_height() as f64,
                );

                self.piston_image
                    .src_rect(self.tilesheet.tile_rect(tile))
                    .draw(
                        &self.map_tiles_texture,
                        &piston_window::DrawState::default(),
                        trans,
                        renderer,
                    );
            }
        }

        let hero_trans = context.transform.trans(300.0, 300.0);
        self.hero.draw(
            "walk",
            self.controller.game_time_s,
            hero_trans,
            renderer,
        );
    }
}
