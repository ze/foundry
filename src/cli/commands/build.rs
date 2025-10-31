use anyhow::{Ok, Result};

use crate::cli::commands::util::current_project;

pub fn build() -> Result<()> {
  let project = current_project()?;
  project.build()?;

  Ok(())
}
