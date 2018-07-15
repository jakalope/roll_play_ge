use std::*;
use piston_window;
use gfx_core;
use tilesheet;
use piston_window::*;
use controller;
use actor;
use game_network;

#[derive(Debug)]
pub enum NewGameError {
    TilesheetError(tilesheet::TilesheetError),
    WindowError(gfx_core::factory::CombinedError),
    HeroError(String),
    NetworkError(game_network::msg::CommError),
}

pub struct Game {
    tilesheet: tilesheet::Tilesheet,
    piston_image: piston_window::Image,
    map_tiles_texture: piston_window::G2dTexture,

    /// Total elapsed game time.
    game_time_s: f64,

    controller: controller::Controller,
    hero: actor::Actor,
    glyphs: piston_window::Glyphs,
    network: game_network::client::Client,
}

impl Game {
    pub fn from_path(
        asset_path: &path::Path,
        window: &mut piston_window::PistonWindow,
        server_address: net::SocketAddr,
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

        // Network
        let network = game_network::client::Client::connect(
            String::from("some_user"),
            String::from("some_password"),
            server_address,
        ).map_err(|err| NewGameError::NetworkError(err))?;

        Ok(Game {
            tilesheet: tilesheet,
            piston_image: piston_window::Image::new(),
            map_tiles_texture: map_tiles_texture,
            game_time_s: 0.0,
            controller: controller::Controller::new(),
            hero: hero,
            glyphs: glyphs,
            network: network,
        })
    }

    pub fn next(&mut self, window: &mut piston_window::PistonWindow) -> bool {
        let event = match window.next() {
            Some(e) => e,
            None => {
                return false;
            }
        };

        self.controller.process_event(&event);
        self.game_time_s += self.controller.dt_s;

        // Send self.controller to the server.
        if let Err(_) = self.network.send_controller_input(
            game_network::bitvec::BitVec::from(
                &self.controller.input,
            ),
        )
        {
            // TODO
        }

        // TODO Receive player's world context from server.
        // TODO For now, we'll let the controller directly control our visualization, but we'll
        // need to eventually negotiate their differences.
        self.hero.control(&self.controller, &self.tilesheet);

        window.draw_2d(&event, |context, gfx| {
            self.render(context, gfx);
            Some(())
        });
        true
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
                    // tiled counts from 1; 0 is invalid
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
            self.game_time_s,
            hero_trans,
            renderer,
        ) {
            Err(actor::ActorDrawError::NoSuchName) => panic!(),
            _ => (),
        }
        self.print(context, renderer);
    }
}
