use anyhow::Result;
use enum_dispatch::enum_dispatch;
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};

use crate::{
    bot::{Bot, Response},
    gen_types::*,
};

enum TgTypes {
    Location(Location),
}

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

impl Bot {
    pub async fn get_user_profile_photos(
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

    pub async fn set_chat_photo(&self, chat_id: i64, photo: InputFile) -> Result<bool> {
        let form = [("chat_id", chat_id)];
        let data = match photo {
            InputFile::Bytes(bytes) => Form::new().part("photo", Part::bytes(bytes)),
        };
        let resp = self.post_data("setChatPhoto", form, data).await?;
        let resp = serde_json::from_value(resp.result)?;
        Ok(resp)
    }
}
