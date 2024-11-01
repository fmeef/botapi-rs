use anyhow::Result;
use botapi::{bot::BotBuilder, gen_types::FileData};
use reqwest::multipart::Part;

#[tokio::main]
async fn main() -> Result<()> {
    let token = std::env::var("TOKEN")?;
    let bot = BotBuilder::new(token)?.build();
    bot.delete_webhook(None).await?;
    let mut offset = 0;
    loop {
        let update = bot.get_updates(Some(offset), None, None, None).await?;
        offset = update
            .iter()
            .map(|u| u.get_update_id())
            .max()
            .unwrap_or_default()
            + 1;
        for update in update {
            if let Some(message) = update.get_message() {
                let chat = message.get_chat();
                let filedata = FileData::Part(Part::text("test").file_name("example"));
                println!(
                    "chat_id {} offset {} message {}",
                    chat.get_id(),
                    offset,
                    message.get_text().map(|m| m.to_owned()).unwrap_or_default()
                );
                bot.send_document(
                    None,
                    chat.get_id(),
                    None,
                    filedata,
                    None,
                    Some("sad"),
                    None,
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
