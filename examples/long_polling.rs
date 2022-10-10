use anyhow::Result;
use botapi::{bot::Bot, ext::LongPoller};
use futures_util::stream::StreamExt;
use std::iter::Iterator;

#[tokio::main]
async fn main() -> Result<()> {
    let token = std::env::var("TOKEN")?;
    let bot = Bot::new(token)?;

    let poller = LongPoller::new(&bot);
    let mut res = poller.get_updates().await;

    while let Some(update) = res.next().await {
        update.iter().for_each(|_| println!("update"));
    }
    Ok(())
}
