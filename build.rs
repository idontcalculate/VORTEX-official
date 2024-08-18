use std::env;
use std::path::PathBuf;

fn main() {
    let proto_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("VORTEX");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("Proto path: {:?}", proto_path);
    println!("Output directory: {:?}", out_dir);

    prost_build::Config::new()
        .out_dir(&out_dir)
        .compile_protos(&[proto_path.join("vortex.proto")], &[proto_path.clone()])
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));

    let generated_file = out_dir.join("vortex.rs");

    if generated_file.exists() {
        println!("Generated file found: {:?}", generated_file);
    } else {
        println!("Generated file not found.");
        panic!("Failed to generate vortex.rs file.");
    }
}
