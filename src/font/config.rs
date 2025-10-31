use std::num::NonZeroU16;

use serde::{Deserialize, Serialize};

use crate::font::subfamily::Subfamily;

#[derive(Serialize, Deserialize)]
pub struct Metadata {
  project_name: String,
  font_name: String,
  font_subfamily: Subfamily,
  version: String,
  copyright: String,
}

impl Metadata {
  pub fn new(
    project_name: String,
    font_name: String,
    font_subfamily: Subfamily,
    version: String,
    copyright: String,
  ) -> Self {
    Self {
      project_name,
      font_name,
      font_subfamily,
      version,
      copyright,
    }
  }

  pub fn project_name(&self) -> &str {
    &self.project_name
  }

  pub fn font_name(&self) -> &str {
    &self.font_name
  }

  pub fn font_subfamily(&self) -> Subfamily {
    self.font_subfamily
  }

  pub fn version(&self) -> &str {
    &self.version
  }

  pub fn copyright(&self) -> &str {
    &self.copyright
  }
}

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

#[derive(Serialize, Deserialize)]
pub struct Config {
  metadata: Metadata,
  dimensions: Dimensions,
}

impl Config {
  pub fn new(metadata: Metadata, dimensions: Dimensions) -> Self {
    Self {
      metadata,
      dimensions,
    }
  }

  pub fn metadata(&self) -> &Metadata {
    &self.metadata
  }

  pub fn dimensions(&self) -> &Dimensions {
    &self.dimensions
  }
}
