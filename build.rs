use std::{io::ErrorKind, path::PathBuf, process::Command};

fn main() {
    Command::new("cargo")
        .current_dir("./chessbik")
        .arg("build")
        .arg("--release")
        .arg("--target=wasm32-unknown-unknown")
        .status()
        .unwrap();

    let mut target_p = PathBuf::from("./target/");

    if let Ok(s) = std::env::var("TARGET") {
        target_p.push(s);
    }

    target_p.push(std::env::var("PROFILE").unwrap());

    if let Err(err) = std::fs::create_dir_all(
        target_p
            .join("static"),
    ) {
        if err.kind() != ErrorKind::AlreadyExists {
            panic!("failure creating static dir:\n{:?}", err);
        }
    }

    Command::new("wasm-bindgen")
        .arg(format!(
            "--out-dir={}/static",
            target_p.display()
        ))
        .arg("--target=web")
        .arg("./chessbik/target/wasm32-unknown-unknown/release/chessbik.wasm")
        .status()
        .unwrap();

    Command::new("wasm-opt")
        .arg("--Oz")
        .arg(format!(
            "{}/static/chessbik_bg.wasm",
            target_p.display()
        ))
        .status()
        .unwrap();

    Command::new("gzip")
        .arg("--best")
        .arg("--force")
        .arg(format!(
            "{}/static/chessbik_bg.wasm",
            target_p.display()
        ))
        .status()
        .unwrap();

        
    for f in std::fs::read_dir("./www/").unwrap() {
        let f = f.unwrap().path();

        if let Err(err) = std::fs::copy(
            f.clone(),
            target_p
                .join("static")
                .join(f.file_name().unwrap()),
        ) {
            if err.kind() != ErrorKind::AlreadyExists {
                panic!("failure copying files");
            }
        }
    }

    if let Err(err) = std::fs::create_dir_all(
        target_p
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
            target_p
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
