use std::fs;

use anyhow::Result;
use std::process::Command;
use tggen::Generate;

fn main() -> Result<()> {
    let json = fs::read_to_string("./telegram-bot-api-spec/api.json")?;

    let gen = Generate::new(json)?;
    let types = gen.generate_types()?;
    let methods = gen.generate_methods()?;

    fs::write("./src/gen_methods.rs", methods)?;

    fs::write("./src/gen_types.rs", types)?;

    if let Err(_) = Command::new("rustfmt")
        .args(["--edition", "2021", "./src/gen_methods.rs"])
        .spawn()
    {
        println!("rustfmt not installed, skipping");
    }

    if let Err(_) = Command::new("rustfmt")
        .args(["--edition", "2021", "./src/gen_types.rs"])
        .spawn()
    {
        println!("rustfmt not installed, skipping");
    }
    Ok(())
}
