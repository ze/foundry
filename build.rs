use std::process::Command;

use which::which;

fn main() {
  println!("cargo::rerun-if-changed=frontend/src");
  println!("cargo::rerun-if-changed=frontend/package.json");
  println!("cargo::rerun-if-changed=frontend/vite.config.ts");
  println!("cargo::rerun-if-changed=frontend/index.html");
  let program = which("npm").expect("Failed to find npm");
  let output = Command::new(program)
    .args(["run", "build"])
    .current_dir("frontend")
    .output()
    .expect("Failed to execute command");

  if !output.status.success() {
    panic!("Command executed with failing error code");
  }
}
