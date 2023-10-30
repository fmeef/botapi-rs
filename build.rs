use std::fs;

use anyhow::Result;
use std::process::Command;
use std::sync::Mutex;
use tggen::Generate;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=generate/");
    println!("cargo:rerun-if-changed=telegram-bot-api-spec/");
    let mtx = Mutex::new(());
    let guard = mtx.lock().unwrap();
    let json = fs::read_to_string("./telegram-bot-api-spec/api.json")?;

    let gen = Generate::new(json)?;
    let types = gen.generate_types()?;
    let methods = gen.generate_methods()?;
    let methods_path = std::env::var("OUT_DIR").unwrap() + "/gen_methods.rs";
    let types_path = std::env::var("OUT_DIR").unwrap() + "/gen_types.rs";
    fs::write(&types_path, types)?;

    fs::write(&methods_path, methods)?;

    match Command::new("rustfmt")
        .args(["--edition", "2021", &methods_path])
        .spawn()
    {
        Err(_) => {
            //println!("rustfmt not installed, skipping");
        }
        Ok(mut handle) => {
            handle.wait().unwrap();
        }
    }

    match Command::new("rustfmt")
        .args(["--edition", "2021", &types_path])
        .spawn()
    {
        Err(_) => {
            //println!("rustfmt not installed, skipping");
        }
        Ok(mut handle) => {
            handle.wait().unwrap();
        }
    }
    drop(guard);
    Ok(())
}
