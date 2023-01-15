use std::fs;

use anyhow::Result;
use std::process::Command;
use tggen::Generate;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=generate/");
    println!("cargo:rerun-if-changed=telegram-bot-api-spec/");
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
            let status = handle.wait().unwrap().success();
            assert!(status);
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
            let status = handle.wait().unwrap().success();
            assert!(status);
        }
    }
    Ok(())
}
