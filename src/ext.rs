use std::net::SocketAddr;
use std::{net::IpAddr, pin::Pin};
use tokio::sync::mpsc;

use crate::{bot::Bot, gen_types::Update};
use anyhow::Result;
use anyhow::{anyhow, Error};
use async_stream::stream;
use futures_core::Stream;
use hyper::body::to_bytes;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

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

pub enum BotUrl {
    Address(String, IpAddr),
    Host(String),
}

pub struct Webhook {
    bot: Bot,
    url: BotUrl,
    drop_pending_updates: bool,
    addr: SocketAddr,
}

impl Webhook {
    pub fn new(bot: &Bot, url: BotUrl, drop_pending_updates: bool, addr: SocketAddr) -> Self {
        Self {
            bot: bot.clone(),
            url,
            drop_pending_updates,
            addr,
        }
    }

    async fn setup(&self) -> Result<bool> {
        match self.url {
            BotUrl::Address(ref addr, ip) => {
                self.bot
                    .set_webhook(
                        addr.to_owned(),
                        None,
                        Some(ip.to_string()),
                        None,
                        None,
                        Some(self.drop_pending_updates),
                        None,
                    )
                    .await
            }
            BotUrl::Host(ref host) => {
                self.bot
                    .set_webhook(
                        host.to_owned(),
                        None,
                        None,
                        None,
                        None,
                        Some(self.drop_pending_updates),
                        None,
                    )
                    .await
            }
        }
    }

    async fn teardown(&self) -> Result<bool> {
        self.bot
            .delete_webhook(Some(self.drop_pending_updates))
            .await
    }

    pub async fn get_updates(self) -> Result<Pin<Box<impl Stream<Item = Result<Update, Error>>>>> {
        let (tx, mut rx) = mpsc::channel(128);
        let svc = make_service_fn(move |_: &AddrStream| {
            let tx = tx.clone();
            async move {
                Ok::<_, Error>(service_fn(move |body: Request<Body>| {
                    let tx = tx.clone();
                    async move {
                        let json = to_bytes(body).await?;

                        if let Ok(update) = serde_json::from_slice::<Update>(&json) {
                            tx.send(update).await?;
                        }
                        Ok::<_, Error>(Response::new(Body::from("")))
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
                yield Err(anyhow!(err));
            }

            self.teardown().await?;
        };

        Ok(Box::pin(s))
    }
}
