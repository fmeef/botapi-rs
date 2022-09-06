use std::ops::Deref;

use anyhow::Result;
use botapi::bot::Bot;

#[tokio::main]
async fn main() -> Result<()> {
    let token = std::env::var("TOKEN")?;
    let bot = Bot::new(token)?;
    loop {
        let update = bot.get_updates(Some(0), Some(1), Some(10), None).await?;
        update.iter().for_each(|update| {
            if let Some(message) = update.get_message() {
                let chat = message.get_chat();
            }
        });
    }
}
