#![warn(clippy::pedantic)]
#![allow(clippy::similar_names)]

use anyhow::Result;

use crate::cli::handler::Handler;

mod cli;
mod font;
mod server;

fn main() -> Result<()> {
  Handler::run()
}
