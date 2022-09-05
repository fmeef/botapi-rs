use anyhow::Result;

use anyhow::anyhow;
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};

use crate::{bot::Bot, gen_types::*};

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    #[serde(rename = "longitude")]
    longitude: f64,
    #[serde(rename = "latitude")]
    latitude: f64,
    #[serde(rename = "horizontal_accuracy")]
    horizontal_accuracy: Option<f64>,
    #[serde(rename = "live_period")]
    live_period: Option<i64>,
    #[serde(rename = "heading")]
    heading: Option<i64>,
    #[serde(rename = "proximity_alert_radius")]
    proximity_alert_radius: Option<i64>,
}

#[doc = "Represents a photo to be sent."]
#[derive(Serialize, Deserialize, Debug)]
pub struct InputMediaPhoto {
    #[doc = "Type of the result, must be photo"]
    #[serde(rename = "type")]
    tg_type: String,
    #[doc = "File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass \"attach://<file_attach_name>\" to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files: https://core.telegram.org/bots/api#sending-files"]
    #[serde(rename = "media")]
    media: Option<InputFile>,
    #[doc = "Optional. Caption of the photo to be sent, 0-1024 characters after entities parsing"]
    #[serde(rename = "caption")]
    caption: Option<String>,
    #[doc = "Optional. Mode for parsing entities in the photo caption. See formatting options for more details."]
    #[serde(rename = "parse_mode")]
    parse_mode: Option<String>,
    #[doc = "Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode"]
    #[serde(rename = "caption_entities")]
    caption_entities: Option<Vec<MessageEntity>>,
}

#[allow(dead_code)]
impl InputMediaPhoto {
    #[doc = "Type of the result, must be photo"]
    pub fn get_tg_type<'a>(&'a self) -> &'a String {
        &self.tg_type
    }
    #[doc = "File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass \"attach://<file_attach_name>\" to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files: https://core.telegram.org/bots/api#sending-files"]
    pub fn get_media<'a>(&'a self) -> &'a Option<InputFile> {
        &self.media
    }
    #[doc = "Optional. Caption of the photo to be sent, 0-1024 characters after entities parsing"]
    pub fn get_caption<'a>(&'a self) -> &'a Option<String> {
        &self.caption
    }
    #[doc = "Optional. Mode for parsing entities in the photo caption. See formatting options for more details."]
    pub fn get_parse_mode<'a>(&'a self) -> &'a Option<String> {
        &self.parse_mode
    }
    #[doc = "Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode"]
    pub fn get_caption_entities<'a>(&'a self) -> &'a Option<Vec<MessageEntity>> {
        &self.caption_entities
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct SendStickerOpts {
    chat_id: EChatId,
    sticker: serde_json::Value,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<i64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: serde_json::Value,
}

impl InputMediaPhoto {
    fn get_params(self, data: Form) -> Result<(Form, serde_json::Value)> {
        let ser = serde_json::to_value(&self)?;
        let res = match self.media {
            Some(InputFile::Bytes(FileBytes {
                name,
                bytes: Some(bytes),
            })) => {
                let form = data.part(name, Part::bytes(bytes));
                Ok(form)
            }
            Some(InputFile::String(string)) => Ok(data),
            _ => Err(anyhow!("cry")),
        }?;
        Ok((res, ser))
    }
}

impl Bot {
    pub async fn send_sticker_ex(
        &self,
        chat_id: EChatId,
        sticker: FileData,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<EReplyMarkup>,
    ) -> Result<Message> {
        let inputfile = sticker.to_inputfile("sticker".to_owned());
        let (data, json) = inputfile.to_form(Form::new())?;
        let form = SendStickerOpts {
            chat_id: chat_id,
            sticker: json,
            disable_notification: disable_notification,
            protect_content: protect_content,
            reply_to_message_id: reply_to_message_id,
            allow_sending_without_reply: allow_sending_without_reply,
            reply_markup: serde_json::to_value(&reply_markup)?,
        };
        let resp = self.post_data("sendSticker", form, data).await?;
        let resp = serde_json::from_value(resp.result)?;
        Ok(resp)
    }

    pub async fn send_sticker_nofile(
        &self,
        chat_id: EChatId,
        sticker: InputFile,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_to_message_id: Option<i64>,
        allow_sending_without_reply: Option<bool>,
        reply_markup: Option<EReplyMarkup>,
    ) -> Result<Message> {
        let form = SendStickerOpts {
            chat_id: chat_id,
            sticker: serde_json::to_value(&sticker)?,
            disable_notification: disable_notification,
            protect_content: protect_content,
            reply_to_message_id: reply_to_message_id,
            allow_sending_without_reply: allow_sending_without_reply,
            reply_markup: serde_json::to_value(&reply_markup)?,
        };
        let resp = self.post("sendSticker", form).await?;
        let resp = serde_json::from_value(resp.result)?;
        Ok(resp)
    }
}
