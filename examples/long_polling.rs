use anyhow::Result;
use botapi::{bot::BotBuilder, ext::LongPoller};
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let token = std::env::var("TOKEN")?;
    let bot = BotBuilder::new(token)?.build();
    bot.delete_webhook(None).await?;
    let poller = LongPoller::new(&bot, None);
    let mut res = poller.get_updates().await;

    while let Some(Ok(update)) = res.next().await {
        println!("update {:?}", update);
    }
    Ok(())
}
