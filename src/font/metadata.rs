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
