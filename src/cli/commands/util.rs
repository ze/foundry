use anyhow::{Context, Ok, Result};

use crate::font::project::Project;

pub fn current_project() -> Result<Project> {
  let path = &std::env::current_dir().context("Could not get current directory")?;
  let project = Project::load(path);
  Ok(project)
}
