use std::*;
use piston_window;
use gfx_core::*;
use gfx_device_gl;
use tilesheet;
use piston_window::Transformed;

#[derive(Debug)]
pub enum NewGameError {
    TilesheetError(tilesheet::TilesheetError),
    WindowError(factory::CombinedError),
}

pub struct Game {
    tilesheet: tilesheet::Tilesheet,
    piston_image: piston_window::Image,
    map_texture: piston_window::Texture<gfx_device_gl::Resources>,
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
        })
    }

    pub fn next(&mut self, window: &mut piston_window::PistonWindow) -> bool {
        let event = match window.next() {
            Some(e) => e,
            None => {
                return false;
            }
        };
        window.draw_2d(&event, |context, gfx| {
            self.render(context, gfx);
            Some(())
        });
        true
    }

    fn render(&mut self, context: piston_window::Context, renderer: &mut piston_window::G2d) {
        piston_window::clear([0.5; 4], renderer);

        for (y, row) in self.tilesheet.layer_tile_iter(0).enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                if tile == 0 {
                    continue;
                }

                let trans = context.transform.trans(
                    x as f64 * self.tilesheet.tile_width() as f64,
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
