// build.rs

use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let _out_dir = env::var("OUT_DIR").unwrap();
    let frontend_dir = format!("{}/frontend", env::var("CARGO_MANIFEST_DIR").unwrap());

    Command::new("npm")
        .args(&["run", "build"])
        .current_dir(&Path::new(&frontend_dir))
        .status()
        .unwrap();

    println!("cargo::rerun-if-changed=fronted");
}
