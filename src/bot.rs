use std::{error::Error, sync::Arc, time::Duration};

use crate::gen_types::ResponseParameters;
use anyhow::Result;

use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

pub use reqwest::multipart::Part;
static TELEGRAM_API: &str = "https://api.telegram.org";

/// Hardcoded serde_json "Response" from telegram bot api. We can't genearate this so declare it
/// here
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub ok: bool,
    pub result: Option<serde_json::Value>,
    pub error_code: Option<i64>,
    pub description: Option<String>,
    pub parameters: Option<ResponseParameters>,
    #[serde(skip, default)]
    pub floods: Option<Vec<ResponseFlood>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseFlood {
    pub ok: bool,
    pub result: Option<serde_json::Value>,
    pub error_code: Option<i64>,
    pub description: Option<String>,
    pub parameters: Option<ResponseParameters>,
}

impl Response {
    /// If this response as a retry_after parameter, async wait this many seconds
    /// returns true if a retry is required
    pub async fn wait(&self) -> bool {
        if let Some(ref params) = self.parameters {
            if let Some(retry) = params.get_retry_after() {
                tokio::time::sleep(Duration::from_secs(retry as u64)).await;
                return true;
            }
        }
        false
    }

    fn get_flood(&self) -> ResponseFlood {
        ResponseFlood {
            ok: self.ok,
            result: self.result.clone(),
            error_code: self.error_code.clone(),
            description: self.description.clone(),
            parameters: self.parameters.clone(),
        }
    }
}

/// Default result type retruned by API calls.
pub type BotResult<T> = Result<T, ApiError>;

/// Nifty telegram bot api wrapper autogenerated from online documentation
#[derive(Debug)]
struct BotState {
    client: reqwest::Client,
    token: String,
    auto_wait: bool,
}

#[derive(Debug)]
enum ErrResponse {
    Response(Response),
    Err(anyhow::Error),
}

/// Error type containing either a Response type from telegram api or a generic error
#[derive(Debug)]
pub struct ApiError(ErrResponse);

impl ApiError {
    pub(crate) fn from_response(resp: Response) -> Self {
        Self(ErrResponse::Response(resp))
    }

    /// Get the telegram api response if it exists, None if this error is a
    /// non-telegram error
    pub fn get_response<'a>(&'a self) -> Option<&'a Response> {
        if let ErrResponse::Response(ref response) = self.0 {
            Some(response)
        } else {
            None
        }
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(value: anyhow::Error) -> Self {
        Self(ErrResponse::Err(value))
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(value: reqwest::Error) -> Self {
        Self(ErrResponse::Err(anyhow::anyhow!(value)))
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(value: serde_json::Error) -> Self {
        Self(ErrResponse::Err(anyhow::anyhow!(value)))
    }
}

impl Error for ApiError {}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            ErrResponse::Response(Response {
                description: Some(ref d),
                ..
            }) => f.write_str(&d)?,
            ErrResponse::Response(Response { error_code, .. }) => {
                f.write_str(&error_code.unwrap_or(-1).to_string())?
            }
            ErrResponse::Err(ref err) => f.write_str(&err.to_string())?,
        };
        Ok(())
    }
}

/// Type holding an active connection to telegram API
#[derive(Debug)]
pub struct Bot(Arc<BotState>);

impl Default for Response {
    fn default() -> Self {
        Response {
            ok: true,
            result: None,
            error_code: None,
            description: None,
            parameters: None,
            floods: None,
        }
    }
}

impl Bot {
    /// Instantiate bot using token optionally enabling autoretry on flood wait
    pub fn new_auto_wait<T>(token: T, auto_wait: bool) -> Result<Self>
    where
        T: Into<String>,
    {
        let client = reqwest::ClientBuilder::new().https_only(true).build()?;
        Ok(Self(Arc::new(BotState {
            client,
            token: token.into(),
            auto_wait,
        })))
    }

    /// Instantiate bot using token
    pub fn new<T>(token: T) -> Result<Self>
    where
        T: Into<String>,
    {
        Self::new_auto_wait(token, false)
    }

    /// generate an api endpoint from bot token
    fn get_endpoint(&self, endpoint: &str) -> String {
        format!("{}/bot{}/{}", TELEGRAM_API, self.0.token, endpoint)
    }

    /// HTTP post helper with x-www-form-urlencoded body
    pub async fn post<T>(&self, endpoint: &str, body: T) -> BotResult<Response>
    where
        T: Serialize,
    {
        let endpoint = self.get_endpoint(endpoint);
        let mut floods = if self.0.auto_wait {
            Some(Vec::<ResponseFlood>::new())
        } else {
            None
        };
        loop {
            let resp = self
                .0
                .client
                .post(&endpoint)
                .query(&body)
                .send()
                .await
                .map_err(|e| e.without_url())?;
            let bytes = resp.bytes().await?;
            let mut resp: Response = serde_json::from_slice(&bytes)?;
            if self.0.auto_wait && resp.wait().await {
                floods.as_mut().unwrap().push(resp.get_flood());
                continue;
            } else {
                if let Some(floods) = floods {
                    resp.floods = Some(floods);
                }
                return Ok(resp);
            }
        }
    }

    /// HTTP post helper with empty body
    pub async fn post_empty(&self, endpoint: &str) -> BotResult<Response> {
        let endpoint = self.get_endpoint(endpoint);
        let mut floods = if self.0.auto_wait {
            Some(Vec::<ResponseFlood>::new())
        } else {
            None
        };
        loop {
            let resp = self
                .0
                .client
                .post(&endpoint)
                .send()
                .await
                .map_err(|e| e.without_url())?;
            let bytes = resp.bytes().await?;
            let mut resp: Response = serde_json::from_slice(&bytes)?;

            if self.0.auto_wait && resp.wait().await {
                floods.as_mut().unwrap().push(resp.get_flood());
                continue;
            } else {
                if let Some(floods) = floods {
                    resp.floods = Some(floods);
                }
                return Ok(resp);
            }
        }
    }

    /// HTTP post helper with x-www-form-urlencode body and multipart/form-data
    pub async fn post_data<T>(&self, endpoint: &str, body: T, data: Form) -> BotResult<Response>
    where
        T: Serialize,
    {
        let endpoint = self.get_endpoint(endpoint);

        let resp = self
            .0
            .client
            .post(&endpoint)
            .query(&body)
            .multipart(data)
            .send()
            .await
            .map_err(|e| e.without_url())?;
        let bytes = resp.bytes().await?;
        let mut resp: Response = serde_json::from_slice(&bytes)?;
        if self.0.auto_wait {
            resp.wait().await;
            resp.floods = Some(vec![resp.get_flood()]);
        }
        Ok(resp)
    }
}

impl Clone for Bot {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}
