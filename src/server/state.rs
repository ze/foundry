use std::sync::Arc;

use crate::font::project::Project;

#[derive(Clone)]
pub struct ApiState {
  project: Arc<Project>,
}

impl ApiState {
  pub fn new(project: Arc<Project>) -> Self {
    Self { project }
  }

  pub fn project(&self) -> &Project {
    &self.project
  }
}
