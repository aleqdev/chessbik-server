use std::{process::Command, path::Path};

fn main() {
    Command::new("cargo")
        .current_dir("./chessbik")
        .arg("build")
        .arg("--release")
        .arg("--target=wasm32-unknown-unknown")
        .status().unwrap();

    Command::new("wasm-bindgen")
        .arg(format!("--out-dir=./target/{}/", std::env::var("PROFILE").unwrap()))
        .arg("--target=web")
        .arg("./chessbik/target/wasm32-unknown-unknown/release/chessbik.wasm")
        .status().unwrap();

    for f in std::fs::read_dir("./www/").unwrap() {
        let f = f.unwrap().path();

        std::fs::copy(
            f.clone(), 
            Path::new("./target/")
                .join(std::env::var("PROFILE").unwrap())
                .join(f.file_name().unwrap())
        ).unwrap();
    }

    std::fs::create_dir(
        Path::new("./target/")
            .join(std::env::var("PROFILE").unwrap())
            .join("assets")
    ).unwrap();

    for f in std::fs::read_dir("./chessbik/assets").unwrap() {
        let f = f.unwrap().path();

        std::fs::copy(
            f.clone(), 
            Path::new("./target/")
                .join(std::env::var("PROFILE").unwrap())
                .join("assets")
                .join(f.file_name().unwrap())
        ).unwrap();
    }

    println!("cargo:rerun-if-changed=build.rs");
}