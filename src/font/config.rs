use serde::{Deserialize, Serialize};

use crate::font::{dimensions::Dimensions, kerning::Kerning, metadata::Metadata};

#[derive(Serialize, Deserialize)]
pub struct Config {
  metadata: Metadata,
  dimensions: Dimensions,
  kerning: Kerning,
}

impl Config {
  pub fn new(metadata: Metadata, dimensions: Dimensions, kerning: Kerning) -> Self {
    Self {
      metadata,
      dimensions,
      kerning,
    }
  }

  pub fn metadata(&self) -> &Metadata {
    &self.metadata
  }

  pub fn dimensions(&self) -> &Dimensions {
    &self.dimensions
  }

  pub fn kerning(&self) -> &Kerning {
    &self.kerning
  }
}
