use std::{
  fs::{self, File},
  io::Write,
  path::{Path, PathBuf},
};

use anyhow::Result;
use serde::Serialize;

use crate::font::{
  builder::Builder, config::Config, contour::contour, glyphs::Glyph, sheet::Sheet,
};

const CONFIG_JSON: &str = "config.json";
const SHEET_PNG: &str = "sheet.png";
const SAMPLES: &str = "samples";

pub struct Project {
  config: Config,
  sheet: Sheet,
}

impl Project {
  pub fn load(base: &Path) -> Self {
    let config = fs::read(base.join(CONFIG_JSON)).expect("No config.json file found");
    let config: Config =
      serde_json::from_slice(&config).expect("Could not deserialize config.json");

    let sheet = Sheet::new(base.join(SHEET_PNG));

    Self { config, sheet }
  }

  pub fn create(base: &Path, config: &Config) -> Result<()> {
    let base = base.join(config.metadata().project_name());
    std::fs::create_dir(&base)?;

    {
      let config_file = File::create(base.join(CONFIG_JSON))?;
      let formatter = serde_json::ser::PrettyFormatter::with_indent(b"  ");
      let mut serializer = serde_json::Serializer::with_formatter(config_file, formatter);
      config.serialize(&mut serializer)?;
    }

    {
      let image = Sheet::create(config);
      image.save(base.join(SHEET_PNG))?;
    }

    {
      let samples = base.join(SAMPLES);
      std::fs::create_dir(samples)?;
    }

    Ok(())
  }

  pub fn build(&self) -> Result<()> {
    let glyphs = self.read_glyphs()?;

    let kerning = self.config.kerning().kern(&glyphs);

    let mut builder = Builder::new(glyphs, kerning);
    let bytes = builder.build(&self.config)?;

    let mut file = File::create(self.font_path())?;
    file.write_all(&bytes)?;

    Ok(())
  }

  fn read_glyphs(&self) -> Result<Vec<Glyph>> {
    let glyph_pixels = self.sheet.read(&self.config)?;
    let glyph_chars = Glyph::glyphs();
    let mut glyphs: Vec<Glyph> = glyph_pixels
      .into_iter()
      .enumerate()
      .filter_map(|(i, pixels)| {
        let character = *glyph_chars.get(i)?;

        if character.is_space() {
          return Some(Glyph::new(character, Vec::new(), Vec::new()));
        }

        let contour = contour(&pixels);
        if contour.is_empty() {
          return None;
        }

        Some(Glyph::new(character, contour, pixels))
      })
      .collect();
    glyphs.sort();

    Ok(glyphs)
  }

  pub fn font_path(&self) -> PathBuf {
    PathBuf::from(format!("{}.ttf", self.config.metadata().font_name()))
  }

  pub fn config(&self) -> &Config {
    &self.config
  }
}
