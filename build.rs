use std::fs;

use anyhow::Result;
use generate::GenerateTypes;

fn main() -> Result<()> {
    let json = fs::read_to_string("./telegram-bot-api-spec/api.json")?;

    let gen = GenerateTypes::new(json)?;
    Ok(())
}
