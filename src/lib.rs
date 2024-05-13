//! Autogenerated telegram bot api wrapper using api spec from <https://github.com/PaulSonOfLars/telegram-bot-api-spec>.
//! Generates a full serde-based telegram bot api wrapper with idiomatic design patterns.

//! # Features
//! - Automatically kept up to date with the latest telegram api
//! - Minimal fluff and boilerplate
//! - Full async support with tokio
//! - Support for both long polling and webhooks
//! - Automatically generated documentation

//! # Examples
//! ## Use webhooks to fetch updates
//! ```no_run
//! use botapi::gen_types::FileData;
//! use botapi::bot::Bot;
//! use botapi::ext::{Webhook, BotUrl};
//! use std::net::{SocketAddr, Ipv4Addr, IpAddr};
//! use futures_util::StreamExt;
//! # tokio_test::block_on(async {
//! let client = Bot::new("sometoken").unwrap();
//! Webhook::new(
//!    &client,
//!    BotUrl::Host("example.com".to_owned()),
//!    false,
//!    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
//!    None,
//! )
//! .get_updates()
//! .await.unwrap()
//! .for_each_concurrent(
//!     None,
//!     |update| async move {
//!         //handle update
//!     },
//! );
//! })
//! ```
//! ## Send messages or media
//! ```no_run
//! use botapi::gen_types::{FileData, Message, ReplyParametersBuilder};
//! use botapi::bot::Bot;
//! # tokio_test::block_on(async {
//! # let message = Message::default();
//! let client = Bot::new("sometoken").unwrap();
//! let bytes = vec![1,2,3];
//! client
//!     .build_send_photo(
//!         message.get_chat().get_id(),
//!            FileData::Bytes(bytes),
//!     )
//!     .caption("If you do not solve this captcha correctly you will be terminated by memetic kill agent")
//!     .reply_parameters(&ReplyParametersBuilder::new(message.get_message_id()).build())
//!     .build()
//!     .await.unwrap();
//! })
//! ```

#![recursion_limit = "256"]
/// Wrapper type for telegram bot api
pub mod bot;
/// Various helpers to manage receiving updates via webhooks or long polling,
/// or to better map json types onto rust design patterns
pub mod ext;

#[allow(unused_imports, rustdoc::bare_urls)]
/// Autogenerated REST api methods
pub mod gen_methods {
    include!("gen_methods.rs");
}
///Autogenerated REST api types from json
#[allow(rustdoc::bare_urls)]
pub mod gen_types {
    include!("gen_types.rs");
}
