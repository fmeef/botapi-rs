use anyhow::Result;
use botapi::{bot::Bot, gen_types::FileData};

#[tokio::main]
async fn main() -> Result<()> {
    let token = std::env::var("TOKEN")?;
    let bot = Bot::new(token)?;
    bot.delete_webhook(None).await?;
    let mut offset = 0;
    loop {
        let update = bot.get_updates(Some(offset), None, None, None).await?;
        offset = update
            .iter()
            .map(|u| *u.get_update_id())
            .max()
            .unwrap_or_default()
            + 1;
        for update in update {
            if let Some(message) = update.get_message() {
                let chat = message.get_chat();
                let filedata = FileData::Bytes("whfwejmkbfejbfe".as_bytes().to_owned());
                println!(
                    "chat_id {} offset {} message {}",
                    chat.get_id(),
                    offset,
                    message
                        .get_text()
                        .clone()
                        .map(|m| m.to_owned())
                        .unwrap_or_default()
                );
                bot.send_document(
                    *chat.get_id(),
                    filedata,
                    None,
                    Some(&"sad".to_owned()),
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
