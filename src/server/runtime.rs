use std::{sync::Arc, time::Duration};

use anyhow::Result;
use tokio::runtime::Runtime;

use crate::{font::project::Project, server::service::start_service};

pub fn start_runtime(project: Project) -> Result<()> {
  let project = Arc::new(project);

  let runtime = Runtime::new()?;
  let service = start_service(project);
  runtime.block_on(service);

  // https://github.com/tokio-rs/tokio/issues/2466
  runtime.shutdown_timeout(Duration::ZERO);

  Ok(())
}
