use std::{fs, io};

const COPILED_FILE: &str = "target/release/libsleep_timer.so";
const NODE_FILE: &str = "index.node";

fn main() {
  let _ret = cp(COPILED_FILE, NODE_FILE).unwrap();

  println!("cargo:rustc-cdylib-link-arg=-undefined");
  if cfg!(target_os = "macos") {
    println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");
  }
}

fn cp(from: &str, to: &str) -> Result<u64, io::Error> {
  let nice = fs::copy(from, to); // Copy foo.txt to bar.txt
  nice
}
