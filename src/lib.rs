//! # postmark-client
//!
//! Postmark client is a "client" agnostic rust client to postmark. We
//! Provide a `reqwest` implementation of a client that can be used pretty
//! simply by initializing it and passing it into the execute function of an
//! [`Endpoint`].
//!
//! Some Endpoints are already provided to you. But if you need some that are
//! not implemented you are not constrained to modified this crated, you can
//! implement your own by implementing the [`Endpoint`] trait and it will
//! work transparently with this library.
//!
//! To use the [`reqwest`] based client, you need to enable the feature `"reqwest"`
//! You can also implement you own client by implementing the [`Client`] trait
//!
//! This crate is heavily inspired by the article ["Designing Rust bindings for REST APIs](https://plume.benboeckel.net/~/JustAnotherBlog/designing-rust-bindings-for-rest-ap-is)
//! by Ben Boeckel and used in the [gitlab](https://crates.io/crates/gitlab) crate.
//! It allows to have modular clients (someone wants to use something else than
//! reqwest), and [`Endpoint`]s not supported by the library without needing to fork it.
//!
//! # Example:
//! ```
//! use postmark::reqwest::PostmarkClient;
//! use postmark::*;
//!
//! # async fn send_email(){
//! let client = PostmarkClient::builder()
//!   .base_url("https://api.postmarkapp.com/")
//!   .token("<sometoken>")
//!   .build();
//!
//! let req = api::email::SendEmailRequest::builder()
//!   .from("me@example.com")
//!   .to("you@example.com")
//!   .body(api::email::Body::Text("Hi, this is me!".to_string()))
//!   .build();
//! let resp = req.execute(&client).await;
//! resp.unwrap();
//! # }
//! ```

/// POSTMARK_API_URL is the default url to poke Postmark's API
pub const POSTMARK_API_URL: &str = "https://api.postmarkapp.com/";

pub mod api;
mod client;

pub use client::*;

#[cfg(feature = "reqwest")]
pub mod reqwest;
