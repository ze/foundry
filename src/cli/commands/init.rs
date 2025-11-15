use std::{num::NonZeroU16, path::Path};

use anyhow::{Context, Result};
use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use strum::IntoEnumIterator;

use crate::font::{
  config::Config, dimensions::Dimensions, kerning::Kerning, metadata::Metadata, project::Project,
  subfamily::Subfamily,
};

pub fn init() -> Result<()> {
  let theme = &ColorfulTheme::default();

  let project_name: String = Input::with_theme(theme)
    .with_prompt("Project name")
    .interact_text()?;

  let font_name: String = Input::with_theme(theme)
    .with_prompt("Font name")
    .default(project_name.clone())
    .interact_text()?;

  let subfamilies: Vec<_> = Subfamily::iter().map(|s| s.to_string()).collect();
  let font_subfamily = Select::with_theme(theme)
    .with_prompt("Font subfamily")
    .items(&subfamilies)
    .default(Subfamily::Regular as usize)
    .interact()?;
  let font_subfamily: Subfamily = Subfamily::iter()
    .nth(font_subfamily)
    .context("Unknown subfamily")?;

  let version: String = Input::with_theme(theme)
    .with_prompt("Version")
    .default("1.0".to_string())
    .interact_text()?;

  let copyright: String = Input::with_theme(theme)
    .with_prompt("Copyright")
    .allow_empty(true)
    .interact_text()?;

  let tile_width: NonZeroU16 = Input::with_theme(theme)
    .with_prompt("Tile width")
    .interact_text()?;

  let ascender_height: NonZeroU16 = Input::with_theme(theme)
    .with_prompt("Ascender height")
    .interact_text()?;

  let descender_height: u16 = Input::with_theme(theme)
    .with_prompt("Descender height")
    .interact_text()?;

  let space_width: NonZeroU16 = Input::with_theme(theme)
    .with_prompt("Space width")
    .default(NonZeroU16::new(5).unwrap())
    .interact_text()?;

  let metadata = Metadata::new(project_name, font_name, font_subfamily, version, copyright);
  let dimensions = Dimensions::new(tile_width, ascender_height, descender_height, space_width);
  let kerning = Kerning::new(true, -1, vec![]);
  let config = Config::new(metadata, dimensions, kerning);

  let config_json = serde_json::to_string_pretty(&config)?;
  println!("{config_json}");

  if Confirm::with_theme(theme)
    .with_prompt("Create project?")
    .default(true)
    .interact()?
  {
    let base: &Path = &std::env::current_dir().context("Could not get current directory")?;
    Project::create(base, &config)?;
  }

  Ok(())
}
