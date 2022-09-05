use crate::gen_types::ResponseParameters;
use anyhow::Result;
use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

static TELEGRAM_API: &str = "https://api.telegram.org";

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub ok: bool,
    pub result: Option<serde_json::Value>,
    pub error_code: Option<i64>,
    pub description: Option<String>,
    pub parameters: Option<ResponseParameters>,
}

pub struct Bot {
    client: reqwest::Client,
    token: String,
}

impl Default for Response {
    fn default() -> Self {
        Response {
            ok: true,
            result: None,
            error_code: None,
            description: None,
            parameters: None,
        }
    }
}

impl Bot {
    pub fn new<T>(token: T) -> Result<Self>
    where
        T: Into<String>,
    {
        let client = reqwest::ClientBuilder::new().https_only(true).build()?;
        Ok(Self {
            client,
            token: token.into(),
        })
    }

    fn get_endpoint(&self, endpoint: &str) -> String {
        format!("{}/bot{}/{}", TELEGRAM_API, self.token, endpoint)
    }

    pub async fn post<T>(&self, endpoint: &str, body: T) -> Result<Response>
    where
        T: Serialize,
    {
        let endpoint = self.get_endpoint(endpoint);
        let resp = self.client.post(endpoint).form(&body).send().await?;
        let resp: Response = serde_json::from_slice(&resp.bytes().await?)?;
        Ok(resp)
    }

    pub async fn post_empty(&self, endpoint: &str) -> Result<Response> {
        let endpoint = self.get_endpoint(endpoint);
        let resp = self.client.post(endpoint).send().await?;
        let resp: Response = serde_json::from_slice(&resp.bytes().await?)?;
        Ok(resp)
    }

    pub async fn post_data<T>(&self, endpoint: &str, body: T, data: Form) -> Result<Response>
    where
        T: Serialize,
    {
        let endpoint = self.get_endpoint(endpoint);
        let resp = self
            .client
            .post(endpoint)
            .form(&body)
            .multipart(data)
            .send()
            .await?;
        let resp: Response = serde_json::from_slice(&resp.bytes().await?)?;
        Ok(resp)
    }
}
