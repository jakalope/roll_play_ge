use std::*;
use tiled;
use image;

#[derive(Debug)]
pub enum TilesheetError {
    IoError(io::Error),
    TiledError(tiled::TiledError),
    ImageError(image::ImageError),
    NoSuchGid(u32),
    NoParentPath,
    NoImages,
}

pub struct Tilesheet {
    image: image::RgbaImage,
    map: tiled::Map,
}

fn tileset_image(
    tmx_path: &path::Path,
    map: &tiled::Map,
) -> Result<image::DynamicImage, TilesheetError> {
    let tileset = map.get_tileset_by_gid(1).ok_or(
        TilesheetError::NoSuchGid(1),
    )?;

    let asset_path = tmx_path.parent().ok_or(TilesheetError::NoParentPath)?;
    let first_image = tileset.images.first().ok_or(TilesheetError::NoImages)?;
    let tilesheet_path = asset_path.join(&first_image.source);

    image::open(&tilesheet_path).map_err(|e| TilesheetError::ImageError(e))
}

impl Tilesheet {
    pub fn from_path(tmx_path: &path::Path) -> Result<Self, TilesheetError> {
        let tmx_file = fs::File::open(tmx_path).map_err(
            |e| TilesheetError::IoError(e),
        )?;

        let map = tiled::parse(tmx_file).map_err(
            |e| TilesheetError::TiledError(e),
        )?;

        let image = tileset_image(tmx_path, &map)?;

        Ok(Tilesheet {
            image: image.to_rgba(),
            map: map,
        })
    }

    pub fn layer_tile_iter(&self, layer_number: usize) -> slice::Iter<Vec<u32>> {
        self.map.layers[layer_number].tiles.iter().clone()
    }

    pub fn background_color(&self) -> [f32; 4] {
        let color = self.map.background_colour.unwrap_or(tiled::Colour {
            red: 127,
            green: 127,
            blue: 127,
        });
        [
            color.red as f32 * (1.0 / 256.0),
            color.green as f32 * (1.0 / 256.0),
            color.blue as f32 * (1.0 / 256.0),
            1.0,
        ]
    }

    pub fn image(&self) -> &image::RgbaImage {
        &self.image
    }

    pub fn tile_width(&self) -> u32 {
        self.map.tile_width
    }

    pub fn tile_height(&self) -> u32 {
        self.map.tile_height
    }

    fn map_width_in_tiles(&self) -> u32 {
        self.image.width() / self.tile_width()
    }

    pub fn tile_rect(&self, tile: u32) -> [f64; 4] {
        let tile = tile - 1; // tiled counts from 1
        let x = (tile % self.map_width_in_tiles() * self.tile_width()) as f64;
        let y = (tile / self.map_width_in_tiles() * self.tile_height()) as f64;
        [x, y, self.tile_width() as f64, self.tile_height() as f64]
    }
}
