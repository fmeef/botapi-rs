use anyhow::Result;
use botapi::bot::Bot;
#[tokio::main]
async fn main() -> Result<()> {
    let token = std::env::var("TOKEN")?;
    let bot = Bot::new(token)?;
    let res = bot.get_me().await?;
    Ok(())
}
