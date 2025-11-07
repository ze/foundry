use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::cli::commands::{build, init, server};

#[derive(Parser)]
#[command(name = "foundry")]
pub struct Handler {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Init,
  Build,
  Server,
}

impl Handler {
  pub fn run() -> Result<()> {
    let handler = Handler::parse();

    match handler.command {
      Commands::Init => init(),
      Commands::Build => build(),
      Commands::Server => server(),
    }
  }
}
