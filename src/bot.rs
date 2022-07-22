use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::gen_types::ResponseParameters;

#[derive(Serialize, Deserialize)]
struct Response {
    pub ok: bool,
    pub result: serde_json::Value,
    pub error_code: i64,
    pub descrition: String,
    pub parameters: Option<ResponseParameters>,
}

pub struct Bot {
    client: reqwest::Client,
}

impl Bot {
    pub fn new() -> Result<Self> {
        let client = reqwest::ClientBuilder::new().https_only(true).build()?;
        Ok(Self { client })
    }

    pub async fn post<T>(&self, endpoint: &str, body: T) -> Result<bool>
    where
        T: Serialize,
    {
        //let endpoint = format!("{}/{}")
        Ok(true)
    }
}
