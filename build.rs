use std::process::Command;
fn main() {
  let output = match Command::new("git").args(&["rev-parse", "HEAD"]).output() {
    Ok(o) => o,
    Err(e) => panic!("Error running git command: {}", e),
  };

  let git_hash = String::from_utf8(output.stdout).unwrap();
  println!("cargo:rustc-env=GIT_HASH={}", git_hash);
}
