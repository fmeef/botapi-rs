use anyhow::Result;
use botapi::bot::Bot;
#[tokio::main]
async fn main() -> Result<()> {
    let token = std::env::var("TOKEN")?;
    let bot = Bot::new(token)?;
    let res = bot.get_me().await?;
    let res = bot.get_updates(Some(0), Some(1), Some(10), None).await?;
    Ok(())
}
