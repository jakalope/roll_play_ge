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

    /// Returns an array of 4 floats, designating `tile`s boundaries `(x,y,width,height)` within
    /// the tileset image. Used to render a tile.
    /// Note: This is NOT in map coordinates.
    pub fn tile_rect(&self, tile: u32) -> [f64; 4] {
        let tile = tile - 1; // tiled counts from 1; we want from 0
        let x = (tile % self.map_width_in_tiles() * self.tile_width()) as f64;
        let y = (tile / self.map_width_in_tiles() * self.tile_height()) as f64;
        [x, y, self.tile_width() as f64, self.tile_height() as f64]
    }

    fn tile_id_from_map_coordinate(&self, layer_index: usize, x: f32, y: f32) -> Option<u32> {
        let layer = self.map.layers.get(layer_index)?;
        let row_index = (y / self.tile_height() as f32).trunc() as usize;
        let row = layer.tiles.get(row_index)?;
        let col_index = (x / self.tile_width() as f32).trunc() as usize;
        match row.get(col_index) {
            Some(value) => Some(*value),
            None => None,
        }
    }

    /// Robustification helper that converts any valid PropertyValue into a bool.
    fn bool_property(value: &tiled::PropertyValue) -> bool {
        match value {
            &tiled::PropertyValue::BoolValue(b) => b,
            &tiled::PropertyValue::IntValue(i) => i != 0,
            &tiled::PropertyValue::FloatValue(f) => f != 0.0,
            &tiled::PropertyValue::ColorValue(c) => c != 0,
            &tiled::PropertyValue::StringValue(ref s) => s != "false",
        }
    }

    pub fn is_walkable(&self, layer_index: usize, x: f32, y: f32) -> bool {
        // get tile ID for map(x,y)
        let tile_gid = match self.tile_id_from_map_coordinate(layer_index, x, y) {
            Some(t) => t,
            None => {
                return true;
            }
        };
        // get the tileset index
        let tileset = match self.map.get_tileset_by_gid(tile_gid) {
            Some(value) => value,
            None => {
                return true;
            }
        };
        // get property for the tile ID
        let tile = match tileset.tiles.get(tile_gid as usize) {
            Some(t) => t,
            None => {
                return true;
            }
        };
        match tile.properties.get("walkable") {
            Some(walkable) => Tilesheet::bool_property(walkable),
            None => true,  // assume walkable if no such property is set
        }
    }
}
