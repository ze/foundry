use anyhow::Result;

use crate::{cli::commands::util::current_project, server::runtime::start_runtime};

pub fn server() -> Result<()> {
  let project = current_project()?;
  start_runtime(project)
}
