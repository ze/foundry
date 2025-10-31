use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString};

#[derive(Clone, Copy, EnumString, AsRefStr, EnumIter, Display, Serialize, Deserialize)]
pub enum Subfamily {
  Thin,
  ExtraLight,
  Light,
  Regular,
  Medium,
  SemiBold,
  Bold,
  ExtraBold,
  Black,
}

impl Subfamily {
  pub fn weight_class(self) -> u16 {
    match self {
      Subfamily::Thin => 100,
      Subfamily::ExtraLight => 200,
      Subfamily::Light => 300,
      Subfamily::Regular => 400,
      Subfamily::Medium => 500,
      Subfamily::SemiBold => 600,
      Subfamily::Bold => 700,
      Subfamily::ExtraBold => 800,
      Subfamily::Black => 900,
    }
  }

  pub fn panose_weight(self) -> u8 {
    match self {
      Subfamily::Thin => 2,
      Subfamily::ExtraLight => 3,
      Subfamily::Light => 4,
      Subfamily::Regular => 5,
      Subfamily::Medium => 6,
      Subfamily::SemiBold => 7,
      Subfamily::Bold => 8,
      Subfamily::ExtraBold => 9,
      Subfamily::Black => 10,
    }
  }

  pub fn name(&self) -> &str {
    self.as_ref()
  }
}
