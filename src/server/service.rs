use std::sync::Arc;

use colored::Colorize;
use tokio::{
  io::{self, AsyncBufReadExt, BufReader},
  net::TcpListener,
  sync::Notify,
};

use crate::{font::project::Project, server::router::router};

pub async fn start_service(project: Arc<Project>) {
  let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
  let port = listener.local_addr().unwrap().port();

  let app = router(project.clone());
  let project_name = project.config().metadata().project_name();

  print_banner(project_name, port);

  let exit_notifier = Arc::new(Notify::new());
  let input_handler = input_handler(port, exit_notifier.clone());
  let shutdown_handler = shutdown_handler(exit_notifier);

  tokio::spawn(input_handler);
  axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_handler)
    .await
    .unwrap();
}

async fn shutdown_handler(exit_notifier: Arc<Notify>) {
  let ctrl_c = async {
    tokio::signal::ctrl_c()
      .await
      .expect("Could not catch Ctrl+C");
  };

  let exit = exit_notifier.notified();

  tokio::select! {
    () = ctrl_c => {}
    () = exit => {}
  }
}

async fn input_handler(port: u16, exit_notifier: Arc<Notify>) {
  const OPEN_BROWSER: &str = "o";
  const QUIT: &str = "q";

  let stdin = BufReader::new(io::stdin());
  let mut lines = stdin.lines();

  while let Ok(Some(line)) = lines.next_line().await {
    let input = line.trim().to_ascii_lowercase();
    match input.as_str() {
      OPEN_BROWSER => {
        let url = format!("http://localhost:{port}");
        if webbrowser::open(&url).is_err() {
          println!("Could not open browser");
        }
      }
      QUIT => {
        exit_notifier.notify_waiters();
        return;
      }
      _ => (),
    }
  }
}

fn print_banner(project_name: &str, port: u16) {
  println!("  {}", "FOUNDRY".truecolor(255, 105, 0).bold());
  println!("  {}: {}", "Project".bold(), project_name);
  println!(
    "  {}   {}",
    "Local:".bold(),
    format!("http://localhost:{}/", port.to_string().bold()).cyan(),
  );
}
