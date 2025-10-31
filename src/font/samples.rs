use core::slice::Iter;
use std::{fs, path::PathBuf};

use anyhow::{Ok, Result};

pub struct Samples {
  files: Vec<PathBuf>,
}

impl Samples {
  pub fn load(path: &PathBuf) -> Result<Self> {
    let mut files: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(path)? {
      let entry = entry?;
      let path = entry.path();

      if let Some(ext) = path.extension()
        && ext == "txt"
      {
        files.push(path);
      }
    }

    let samples = Self { files };
    Ok(samples)
  }

  pub fn iter(&self) -> Iter<'_, PathBuf> {
    self.files.iter()
  }
}
