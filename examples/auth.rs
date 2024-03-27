use anyhow::Result;
use botapi::bot::BotBuilder;
#[tokio::main]
async fn main() -> Result<()> {
    let token = std::env::var("TOKEN")?;
    let bot = BotBuilder::new(token)?.build();
    let res = bot.get_me().await?;
    println!("getme: {}", res.get_username().unwrap_or_default());
    let res = bot.get_updates(Some(0), Some(1), Some(10), None).await?;
    res.iter().for_each(|update| {
        println!(
            "got update: {}",
            update.get_message().unwrap().get_text().unwrap()
        );
    });
    Ok(())
}
