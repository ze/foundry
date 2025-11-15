use std::num::NonZeroU16;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Dimensions {
  tile_width: NonZeroU16,
  ascender_height: NonZeroU16,
  descender_height: u16,
  space_width: NonZeroU16,
}

impl Dimensions {
  pub fn new(
    tile_width: NonZeroU16,
    ascender_height: NonZeroU16,
    descender_height: u16,
    space_width: NonZeroU16,
  ) -> Self {
    Self {
      tile_width,
      ascender_height,
      descender_height,
      space_width,
    }
  }

  pub fn tile_width(&self) -> u16 {
    self.tile_width.get()
  }

  pub fn ascender_height(&self) -> u16 {
    self.ascender_height.get()
  }

  pub fn descender_height(&self) -> u16 {
    self.descender_height
  }

  pub fn tile_height(&self) -> u16 {
    self.ascender_height() + self.descender_height()
  }

  pub fn space_width(&self) -> u16 {
    self.space_width.get()
  }
}
