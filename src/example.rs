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

impl InputMediaPhoto {
    fn get_params(self, data: Form) -> Result<(Form, serde_json::Value)> {
        let ser = serde_json::to_value(&self)?;
        let res = match self.media {
            Some(InputFile::Bytes(FileBytes { name, bytes })) => {
                let form = data.part(name, Part::bytes(bytes));
                Ok(form)
            }
            Some(InputFile::String(string)) => Ok(data),
            None => Err(anyhow!("cry")),
        }?;
        Ok((res, ser))
    }
}

impl Bot {
    pub async fn ex_get_user_profile_photos(
        &self,
        user_id: i64,
        offset: i64,
        limit: i64,
    ) -> Result<UserProfilePhotos> {
        let form = [
            ("user_id", user_id.to_string()),
            ("offset", offset.to_string()),
            ("limit", limit.to_string()),
        ];
        let resp = self.post("getUserProfilePhotos", form).await?;
        let resp = serde_json::from_value(resp.result)?;
        Ok(resp)
    }

    pub async fn ex_set_chat_photo(&self, chat_id: i64, photo: InputFile) -> Result<bool> {
        let form = [("chat_id", chat_id)];
        let data = match photo {
            InputFile::Bytes(FileBytes { name, bytes }) => {
                Form::new().part("photo", Part::bytes(bytes))
            }
            InputFile::String(string) => Form::new().part("photo", Part::text(string)),
        };
        let resp = self.post_data("setChatPhoto", form, data).await?;
        let resp = serde_json::from_value(resp.result)?;
        Ok(resp)
    }
}
