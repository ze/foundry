use anyhow::{Ok, Result};
use axum::serve;
use tokio::{net::TcpListener, runtime::Runtime};

use crate::{font::project::Project, server::router::router};

pub fn start_runtime(project: Project) -> Result<()> {
  let runtime = Runtime::new()?;
  runtime.block_on(async {
    let app = router(project);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listener, app).await.unwrap();
  });

  Ok(())
}
