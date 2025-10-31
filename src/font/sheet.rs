use std::path::PathBuf;

use anyhow::Result;
use image::{ImageReader, Rgb, RgbImage};

use crate::font::{config::Config, glyphs::sheet_unicode_blocks, point::Point};

mod colors {
  pub const SHEET_LIGHT: [u8; 3] = [210, 107, 152]; // #D26B98
  pub const SHEET_DARK: [u8; 3] = [173, 58, 108]; // #AD3A6C
  pub const BASELINE_LIGHT: [u8; 3] = [208, 71, 138]; // #D0478A
  pub const BASELINE_DARK: [u8; 3] = [169, 29, 87]; // #A91D57

  pub const INACTIVE_SHEET_LIGHT: [u8; 3] = [180, 143, 113]; // #B48F71
  pub const INACTIVE_SHEET_DARK: [u8; 3] = [144, 98, 67]; // #906243
  pub const INACTIVE_BASELINE_LIGHT: [u8; 3] = [162, 120, 88]; // #A27858
  pub const INACTIVE_BASELINE_DARK: [u8; 3] = [125, 77, 48]; // #7D4D30

  pub const GLYPH_COLOR: [u8; 3] = [0, 0, 0]; // #000000
}

const GLYPHS_MAX: u32 = sheet_unicode_blocks::BASIC_LATIN_SUPPORTED
  + sheet_unicode_blocks::LATIN_SUPPLEMENT_SUPPORTED
  + sheet_unicode_blocks::LATIN_A_SUPPORTED
  + sheet_unicode_blocks::CURRENCY_SUPPORTED
  + sheet_unicode_blocks::NOT_DEF;
const COLUMNS: u32 = 26;
const ROWS: u32 = GLYPHS_MAX.div_ceil(COLUMNS);

pub struct Sheet {
  path: PathBuf,
}

impl Sheet {
  pub fn new(path: PathBuf) -> Self {
    Self { path }
  }

  pub fn create(config: &Config) -> RgbImage {
    let dimensions = config.dimensions();
    let block_width = u32::from(dimensions.tile_width());
    let width = block_width * COLUMNS;

    let ascender_height = u32::from(dimensions.ascender_height());
    let descender_height = u32::from(dimensions.descender_height());
    let block_height = u32::from(dimensions.tile_height());

    let height = block_height * ROWS;

    let mut image = RgbImage::new(width, height);

    for i in 0..width {
      for j in 0..height {
        let column = i / block_width;
        let row = j / block_height;
        let is_width_light = column % 2 == 0;
        let is_height_flipped = row % 2 == 1;
        let is_baseline = descender_height != 0 && j % block_height == ascender_height - 1;
        let is_inactive = (column + (row * COLUMNS)) >= GLYPHS_MAX;
        let color = if is_width_light == is_height_flipped {
          if is_baseline {
            if is_inactive {
              colors::INACTIVE_BASELINE_DARK
            } else {
              colors::BASELINE_DARK
            }
          } else if is_inactive {
            colors::INACTIVE_SHEET_DARK
          } else {
            colors::SHEET_DARK
          }
        } else if is_baseline {
          if is_inactive {
            colors::INACTIVE_BASELINE_LIGHT
          } else {
            colors::BASELINE_LIGHT
          }
        } else if is_inactive {
          colors::INACTIVE_SHEET_LIGHT
        } else {
          colors::SHEET_LIGHT
        };
        let pixel = Rgb(color);
        image.put_pixel(i, j, pixel);
      }
    }

    image
  }

  pub fn read(&self, config: &Config) -> Result<Vec<Vec<Point>>> {
    let image = ImageReader::open(&self.path)?.decode()?;
    let image = image.to_rgb8();

    let tile_width = u32::from(config.dimensions().tile_width());
    let tile_height = u32::from(config.dimensions().tile_height());
    let y_offset: i16 = config.dimensions().ascender_height().try_into()?;

    let mut points = Vec::new();
    for j in 0..ROWS {
      for i in 0..COLUMNS {
        let mut cluster = Vec::new();
        let x_start = i * tile_width;
        let y_start = j * tile_height;
        for y in y_start..(y_start + tile_height) {
          for x in x_start..(x_start + tile_width) {
            let pixel = image.get_pixel(x, y);
            if pixel.0 == colors::GLYPH_COLOR {
              let px: i16 = (x - x_start).try_into()?;

              let y: i16 = y.try_into()?;
              let y_start: i16 = y_start.try_into()?;
              let py = y_offset - (y - y_start) - 1;

              let p = Point::new(px, py);
              cluster.push(p);
            }
          }
        }
        points.push(cluster);
      }
    }

    Ok(points)
  }
}
