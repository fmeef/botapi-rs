use anyhow::Result;
use botapi::{bot::Bot, gen_types::FileData};

#[tokio::main]
async fn main() -> Result<()> {
    let token = std::env::var("TOKEN")?;
    let bot = Bot::new(token)?;
    loop {
        let update = bot.get_updates(Some(0), Some(1), Some(10), None).await?;
        for update in update {
            if let Some(message) = update.get_message() {
                let chat = message.get_chat();
                let filedata = FileData::Bytes(vec![0, 0, 0]);
                bot.send_document(
                    *chat.get_id(),
                    filedata,
                    None,
                    Some("cry".to_owned()),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                )
                .await?;
            }
        }
    }
}
