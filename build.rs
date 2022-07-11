use std::{
    fs::{self, File},
    os,
};

use ::rustfmt::{config::Config, format_input, Input};
use anyhow::Result;
use generate::GenerateTypes;

fn main() -> Result<()> {
    let json = fs::read_to_string("./telegram-bot-api-spec/api.json")?;

    let gen = GenerateTypes::new(json)?;
    let out = gen.generate_types()?;

    fs::write("./src/gen_types.rs", out)?;
    let input = Input::File("./src/gen_types.rs".into());
    let config = Config::default();
    let v = rustfmt::run(input, &config);
    Ok(())
}
