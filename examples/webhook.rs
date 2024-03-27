use anyhow::Result;
use botapi::{
    bot::BotBuilder,
    ext::{BotUrl, Webhook},
};
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let token = std::env::var("TOKEN")?;
    let url = std::env::var("URL")?;

    let url = BotUrl::Host(url);
    let bot = BotBuilder::new(token)?.build();

    let addr = ([0, 0, 0, 0], 8080).into();
    let poller = Webhook::new(&bot, url, false, addr, None);
    let mut res = poller.get_updates().await?;

    while let Some(Ok(update)) = res.next().await {
        println!("update {:?}", update);
    }
    Ok(())
}
