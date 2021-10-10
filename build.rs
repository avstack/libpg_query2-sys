extern crate bindgen;

use std::env;
use std::process::Command;
use std::path::PathBuf;

fn main() {
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

  Command::new("cp")
    .args(&["-r", "./c_libs/libpg_query", &out_path.display().to_string()])
    .output()
    .expect(&format!("Failed to copy libpg_query to OUT_DIR {}", out_path.join("libpg_query").display()));

  Command::new("make")
    .current_dir(format!("{}/libpg_query", out_path.display()))
    .output()
    .expect("Failed to build libpg_query");

  println!("cargo:rerun-if-changed=c_libs/libpg_query");
  println!("cargo:rustc-link-search=native={}", out_path.join("libpg_query").display());
  println!("cargo:rustc-link-lib=static={}", "pg_query");
}
