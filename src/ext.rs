use std::pin::Pin;

use anyhow::Error;
use async_stream::stream;
use futures_core::Stream;

use crate::{bot::Bot, gen_types::Update};

pub struct LongPoller {
    bot: Bot,
    offset: i64,
}

impl LongPoller {
    pub fn new(bot: &Bot) -> Self {
        Self {
            bot: bot.clone(),
            offset: 0,
        }
    }

    pub async fn get_updates(mut self) -> Pin<Box<impl Stream<Item = Result<Update, Error>>>> {
        let s = stream! {
            loop {
                let update = self.bot.get_updates(Some(self.offset), None, None, None).await?;
                let mut max = 0;
                for update in update {
                    let id =  *update.get_update_id();
                    if id > max {
                        max = id;
                    }
                    yield Ok(update);
                }

                self.offset = max + 1;
            }
        };

        Box::pin(s)
    }
}
