use crate::font::project::Project;

pub struct RouterState {
  project: Project,
}

impl RouterState {
  pub fn new(project: Project) -> Self {
    Self { project }
  }

  pub fn project(&self) -> &Project {
    &self.project
  }
}
