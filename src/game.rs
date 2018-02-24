use std::*;
use piston_window;
use gfx_core::*;
use gfx_device_gl;
use tilesheet;
use piston_window::*;
use controller;
use model;

#[derive(Debug)]
pub enum NewGameError {
    TilesheetError(tilesheet::TilesheetError),
    WindowError(factory::CombinedError),
}

pub struct Game {
    tilesheet: tilesheet::Tilesheet,
    piston_image: piston_window::Image,
    map_texture: piston_window::Texture<gfx_device_gl::Resources>,
    controller: controller::Controller,
    model: model::Model,
}

impl Game {
    pub fn from_path(
        asset_path: &path::Path,
        window: &mut piston_window::PistonWindow,
    ) -> Result<Self, NewGameError> {
        let tilesheet = tilesheet::Tilesheet::from_path(&asset_path.join("tiled_base64_zlib.tmx"))
            .map_err(|e| NewGameError::TilesheetError(e))?;

        let tiletexture = piston_window::Texture::from_image(
            &mut window.factory,
            tilesheet.image(),
            &piston_window::TextureSettings::new(),
        ).map_err(|e| NewGameError::WindowError(e))?;

        let piston_image = piston_window::Image::new();

        Ok(Game {
            tilesheet: tilesheet,
            piston_image: piston_image,
            map_texture: tiletexture,
            controller: controller::Controller::new(),
            model: model::Model::new(),
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

        self.update_model();

        // Update view
        window.draw_2d(&event, |context, gfx| {
            self.render(context, gfx);
            Some(())
        });
        true
    }

    fn update_controller(&mut self, event: &piston_window::Event) {
        self.controller.dt_s = match event.idle_args() {
            Some(_args) => _args.dt as f32,
            None => 0.0,
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

    fn update_model(&mut self) {
        self.model.vy = self.controller.walk_rate *
            ((self.controller.input.up as i32) - (self.controller.input.down as i32)) as f32;
        self.model.vx = self.controller.walk_rate *
            ((self.controller.input.left as i32) - (self.controller.input.right as i32)) as f32;
        self.model.y = self.model.y + self.controller.dt_s * self.model.vy;
        self.model.x = self.model.x + self.controller.dt_s * self.model.vx;
    }

    fn render(&mut self, context: piston_window::Context, renderer: &mut piston_window::G2d) {
        piston_window::clear([0.5; 4], renderer);

        for (y, row) in self.tilesheet.layer_tile_iter(0).enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                if tile == 0 {
                    continue;
                }

                let trans = context.transform.trans(
                    self.model.x as f64 -
                        x as f64 * self.tilesheet.tile_width() as f64,
                    self.model.y as f64 -
                        y as f64 * self.tilesheet.tile_height() as f64,
                );

                self.piston_image
                    .src_rect(self.tilesheet.tile_rect(tile))
                    .draw(
                        &self.map_texture,
                        &piston_window::DrawState::default(),
                        trans,
                        renderer,
                    );
            }
        }
    }
}
