use rand::rngs::OsRng;
use rand::RngCore;
use std::net::SocketAddr;
use std::{net::IpAddr, pin::Pin};
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;
use uuid::Uuid;

use crate::bot::ApiError;
use crate::gen_types::UpdateExt;
use crate::{bot::Bot, gen_types::Update};
use anyhow::anyhow;
use anyhow::Result;
use async_stream::stream;
use futures_core::Stream;
use hyper::body::to_bytes;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

/// Helper for fetching updates via long polling.
pub struct LongPoller {
    bot: Bot,
    offset: i64,
    allowed_updates: Option<Vec<String>>,
}

impl LongPoller {
    pub fn new(bot: &Bot, allowed_updates: Option<Vec<String>>) -> Self {
        Self {
            bot: bot.clone(),
            offset: 0,
            allowed_updates,
        }
    }

    /// Return an async stream of updates, terminating with error
    pub async fn get_updates(
        mut self,
    ) -> Pin<Box<impl Stream<Item = Result<UpdateExt, ApiError>>>> {
        let s = stream! {
            loop {
                let update = self.bot.get_updates(Some(self.offset), None, None, self.allowed_updates.as_ref()).await?;
                let mut max = 0;
                for update in update {
                    let id = update.get_update_id();
                    if id > max {
                        max = id;
                    }
                    yield Ok(update.into());
                }

                self.offset = max + 1;
            }
        };

        Box::pin(s)
    }
}

/// Endpoint for webhooks, could be either a raw ip address or a hostname
pub enum BotUrl {
    Address(String, IpAddr),
    Host(String),
}

/// Helper for fetching updates via webhook. This currently requires a reverse proxy as
/// tls is not supported.
pub struct Webhook {
    bot: Bot,
    url: BotUrl,
    drop_pending_updates: bool,
    addr: SocketAddr,
    cookie: Uuid,
    allowed_updates: Option<Vec<String>>,
}

impl Webhook {
    pub fn new(
        bot: &Bot,
        url: BotUrl,
        drop_pending_updates: bool,
        addr: SocketAddr,
        allowed_updates: Option<Vec<String>>,
    ) -> Self {
        let mut bytes: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        OsRng.fill_bytes(&mut bytes);
        let cookie = Uuid::from_slice(bytes.as_slice()).expect("invalid uuid");
        Self {
            bot: bot.clone(),
            url,
            drop_pending_updates,
            addr,
            cookie,
            allowed_updates,
        }
    }

    async fn setup(&self) -> Result<bool, ApiError> {
        match self.url {
            BotUrl::Address(ref addr, ip) => {
                self.bot
                    .set_webhook(
                        &addr,
                        None,
                        Some(&ip.to_string()),
                        None,
                        self.allowed_updates.as_ref(),
                        Some(self.drop_pending_updates),
                        Some(self.cookie.to_string().as_str()),
                    )
                    .await
            }
            BotUrl::Host(ref host) => {
                self.bot
                    .set_webhook(
                        &host,
                        None,
                        None,
                        None,
                        self.allowed_updates.as_ref(),
                        Some(self.drop_pending_updates),
                        Some(self.cookie.to_string().as_str()),
                    )
                    .await
            }
        }
    }

    async fn teardown(&self) -> Result<bool, ApiError> {
        self.bot
            .delete_webhook(Some(self.drop_pending_updates))
            .await
    }

    /// Return an async stream of updates, terminating with error. Webhooks are enabled on
    /// startup and disabled on error.
    pub async fn get_updates(
        self,
    ) -> Result<Pin<Box<impl Stream<Item = Result<UpdateExt, ApiError>>>>, ApiError> {
        let (tx, mut rx) = mpsc::channel(128);
        let cookie = self.cookie.clone();
        let svc = make_service_fn(move |_: &AddrStream| {
            let tx = tx.clone();
            async move {
                Ok::<_, ApiError>(service_fn(move |body: Request<Body>| {
                    let tx = tx.clone();
                    async move {
                        if let Some(token) = body.headers().get("X-Telegram-Bot-Api-Secret-Token") {
                            if token.to_str().unwrap_or("") == cookie.to_string().as_str() {
                                let json = to_bytes(body).await.map_err(|e| anyhow!(e))?;
                                if let Ok(update) = serde_json::from_slice::<Update>(&json) {
                                    tx.send(update.into())
                                        .await
                                        .map_err(|e: SendError<UpdateExt>| anyhow!(e))?;
                                }
                            }
                        }
                        Ok::<_, ApiError>(Response::new(Body::from("")))
                    }
                }))
            }
        });

        let server = Server::bind(&self.addr).serve(svc);

        let fut = tokio::task::spawn(async move { server.await });

        if let Err(err) = self.setup().await {
            self.teardown().await?;
            return Err(err);
        }

        let s = stream! {
            while let Some(update) = rx.recv().await {
                yield Ok(update);
            }
            if let Err(err) = fut.await {
                yield Err(anyhow!(err).into());
            }

            self.teardown().await?;
        };

        Ok(Box::pin(s))
    }
}
