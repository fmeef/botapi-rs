use anyhow::Result;
use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

use crate::gen_types::ResponseParameters;

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub ok: bool,
    pub result: serde_json::Value,
    pub error_code: i64,
    pub descrition: String,
    pub parameters: Option<ResponseParameters>,
}

pub struct Bot {
    client: reqwest::Client,
}

impl Default for Response {
    fn default() -> Self {
        Response {
            ok: true,
            result: serde_json::Value::Null,
            error_code: 0,
            descrition: "".into(),
            parameters: None,
        }
    }
}

impl Bot {
    pub fn new() -> Result<Self> {
        let client = reqwest::ClientBuilder::new().https_only(true).build()?;
        Ok(Self { client })
    }

    pub async fn post<T>(&self, endpoint: &str, body: T) -> Result<Response>
    where
        T: Serialize,
    {
        //TODO:
        Ok(Response::default())
    }

    pub async fn post_data<T>(&self, endpoint: &str, body: T, data: Form) -> Result<Response> {
        Ok(Response::default())
    }
}
