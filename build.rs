use std::{io::ErrorKind, path::Path, process::Command};

fn main() {
    Command::new("cargo")
        .current_dir("./chessbik")
        .arg("build")
        .arg("--release")
        .arg("--target=wasm32-unknown-unknown")
        .status()
        .unwrap();

    if let Err(err) = std::fs::create_dir(
        Path::new("./target/")
            .join(std::env::var("PROFILE").unwrap())
            .join("static"),
    ) {
        if err.kind() != ErrorKind::AlreadyExists {
            panic!("failure creating static dir");
        }
    }

    Command::new("wasm-bindgen")
        .arg(format!(
            "--out-dir=./target/{}/static",
            std::env::var("PROFILE").unwrap()
        ))
        .arg("--target=web")
        .arg("./chessbik/target/wasm32-unknown-unknown/release/chessbik.wasm")
        .status()
        .unwrap();

    for f in std::fs::read_dir("./www/").unwrap() {
        let f = f.unwrap().path();

        if let Err(err) = std::fs::copy(
            f.clone(),
            Path::new("./target/")
                .join(std::env::var("PROFILE").unwrap())
                .join("static")
                .join(f.file_name().unwrap()),
        ) {
            if err.kind() != ErrorKind::AlreadyExists {
                panic!("failure copying files");
            }
        }
    }

    if let Err(err) = std::fs::create_dir(
        Path::new("./target/")
            .join(std::env::var("PROFILE").unwrap())
            .join("static/assets"),
    ) {
        if err.kind() != ErrorKind::AlreadyExists {
            panic!("failure creating assets dir");
        }
    }

    for f in std::fs::read_dir("./chessbik/assets").unwrap() {
        let f = f.unwrap().path();

        if let Err(err) = std::fs::copy(
            f.clone(),
            Path::new("./target/")
                .join(std::env::var("PROFILE").unwrap())
                .join("static/assets")
                .join(f.file_name().unwrap()),
        ) {
            if err.kind() != ErrorKind::AlreadyExists {
                panic!("failure copying files");
            }
        }
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=chessbik/");
}
