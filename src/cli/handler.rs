use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand, arg, command};

use crate::cli::commands::{build, debug, init, server};

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
  Debug {
    #[arg(long)]
    path: PathBuf,
  },
  Server,
}

impl Handler {
  pub fn run() -> Result<()> {
    let handler = Handler::parse();

    match handler.command {
      Commands::Init => init(),
      Commands::Build => build(),
      Commands::Debug { path } => debug(path),
      Commands::Server => server(),
    }
  }
}
