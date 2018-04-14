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
    glyphs: piston_window::Glyphs,
}

impl Game {
    pub fn from_path(
        asset_path: &path::Path,
        window: &mut piston_window::PistonWindow,
    ) -> Result<Self, NewGameError> {
        // Map
        let tilesheet = tilesheet::Tilesheet::from_path(&asset_path.join("tiled_base64_zlib.tmx"))
            .map_err(|e| NewGameError::TilesheetError(e))?;

        let texture_settings = piston_window::TextureSettings::new();
        let map_tiles_texture = piston_window::Texture::from_image(
            &mut window.factory,
            tilesheet.image(),
            &texture_settings,
        ).map_err(|e| NewGameError::WindowError(e))?;

        // Hero
        let hero_texture = piston_window::Texture::from_path(
            &mut window.factory,
            &asset_path.join("hero_walk.png"),
            piston_window::Flip::None,
            &texture_settings,
        ).map_err(|e| NewGameError::HeroError(e))?;

        let hero_sheet = actor::CharacterSheet::from_texture(hero_texture, 3, 4)
            .map_err(|e| NewGameError::HeroError(e))?;

        let hero_walk_chooser = actor::WalkingSpriteChooser::from_sheet(0.2, &hero_sheet)
            .map_err(|e| NewGameError::HeroError(e))?;

        let mut hero = actor::Actor::new();
        hero.insert_chooser(
            String::from("walk"),
            cell::RefCell::<Box<actor::WalkingSpriteChooser>>::new(
                Box::<actor::WalkingSpriteChooser>::new(hero_walk_chooser),
            ),
        );

        // Font
        let ref font = asset_path.join("yoster.ttf");
        let factory = window.factory.clone();
        let glyphs =
            piston_window::Glyphs::new(font, factory, piston_window::TextureSettings::new())
                .unwrap();

        Ok(Game {
            tilesheet: tilesheet,
            piston_image: piston_window::Image::new(),
            map_tiles_texture: map_tiles_texture,
            controller: controller::Controller::new(),
            hero: hero,
            glyphs: glyphs,
        })
    }

    pub fn next(&mut self, window: &mut piston_window::PistonWindow) -> bool {
        let event = match window.next() {
            Some(e) => e,
            None => {
                return false;
            }
        };

        self.update(&event);
        self.hero.control(&self.controller);

        window.draw_2d(&event, |context, gfx| {
            self.render(context, gfx);
            Some(())
        });
        true
    }

    fn update(&mut self, event: &piston_window::Event) {
        if let Some(update_args) = event.update_args() {
            self.controller.dt_s = update_args.dt;
            self.controller.game_time_s += self.controller.dt_s;
        };

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
                    self.controller.input.attack = true;
                }
                keyboard::Key::K => {
                    self.controller.input.defend = true;
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
                    self.controller.input.attack = false;
                }
                keyboard::Key::K => {
                    self.controller.input.defend = false;
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

    fn print(&mut self, context: piston_window::Context, renderer: &mut G2d) {
        rectangle(
            [0.0, 0.0, 0.0, 0.9],
            [0.0, 0.0, 400.0, 100.0], // rectangle
            context.transform,
            renderer,
        );

        let transform = context.transform.trans(8.0, 16.0);
        text::Text::new_color([1.0, 1.0, 1.0, 1.0], 8)
            .draw(
                &(1.0 / self.controller.dt_s).to_string(),
                &mut self.glyphs,
                &context.draw_state,
                transform,
                renderer,
            )
            .unwrap();
    }

    fn render(&mut self, context: piston_window::Context, renderer: &mut G2d) {
        piston_window::clear(self.tilesheet.background_color(), renderer);

        let viewport = match context.viewport {
            Some(viewport) => viewport,
            None => {
                return; // Headless mode?
            }
        };

        let center = [
            viewport.window_size[0] as f64 * 0.5,
            viewport.window_size[1] as f64 * 0.5,
        ];

        for (y, row) in self.tilesheet.layer_tile_iter(0).enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                if tile == 0 {
                    continue;
                }

                let trans = context.transform.trans(
                    self.hero.x as f64 + center[0] -
                        x as f64 * self.tilesheet.tile_width() as f64,
                    self.hero.y as f64 + center[1] -
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

        let hero_trans = context.transform.trans(center[0], center[1]);
        match self.hero.draw(
            "walk",
            self.controller.game_time_s,
            hero_trans,
            renderer,
        ) {
            Err(actor::ActorDrawError::NoSuchName) => panic!(),
            _ => (),
        }
        self.print(context, renderer);
    }
}
