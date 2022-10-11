use anyhow::Result;
use botapi::{bot::Bot, ext::LongPoller};
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let token = std::env::var("TOKEN")?;
    let bot = Bot::new(token)?;

    let poller = LongPoller::new(&bot);
    let mut res = poller.get_updates().await;

    while let Some(Ok(update)) = res.next().await {
        println!("update {}", update.get_update_id());
    }
    Ok(())
}
