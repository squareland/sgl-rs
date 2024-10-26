use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("gamma.rs");

    let table: [f32; 256] = std::array::from_fn(|i| {
        (i as f32 / 255.0).powf(2.2)
    });

    fs::write(
        &dest_path,
        format!("pub const GAMMA: [f32; 256] = {:?};", table)
    ).unwrap();
    println!("cargo::rerun-if-changed=build.rs");
}