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

    fs::write("./src/gen_types.rs", types)?;

    fs::write("./src/gen_methods.rs", methods)?;

    match Command::new("rustfmt")
        .args(["--edition", "2021", "./src/gen_methods.rs"])
        .spawn()
    {
        Err(_) => {
            println!("rustfmt not installed, skipping");
        }
        Ok(mut handle) => {
            handle.wait().unwrap();
        }
    }

    match Command::new("rustfmt")
        .args(["--edition", "2021", "./src/gen_types.rs"])
        .spawn()
    {
        Err(_) => {
            println!("rustfmt not installed, skipping");
        }
        Ok(mut handle) => {
            handle.wait().unwrap();
        }
    }
    drop(guard);
    Ok(())
}
