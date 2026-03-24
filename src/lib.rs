//! Postmark is an HTTP-client-agnostic Rust client for the Postmark API.
//! This crate provides a `reqwest` client implementation that you initialize
//! and pass to `execute` on a [`Query`]. All [`Endpoint`]s implement [`Query`].
//!
//! Many endpoints are provided out of the box. If you need one that is not
//! implemented, you can add your own type implementing [`Endpoint`] and use it
//! with the same client and query flow.
//!
//! To use the [`reqwest`] client, enable the `"reqwest"` feature.
//! You can also implement your own client by implementing [`Client`].
//!
//! This crate is heavily inspired by the article ["Designing Rust bindings for REST APIs"](https://plume.benboeckel.net/~/JustAnotherBlog/designing-rust-bindings-for-rest-ap-is)
//! by Ben Boeckel and used in the [gitlab](https://crates.io/crates/gitlab) crate.
//! It allows to have modular clients (someone wants to use something else than
//! reqwest), and [`Endpoint`]s not supported by the library without needing to fork it.
//!
//! Expanded API coverage now includes: bulk, bounce, templates, servers,
//! message streams, messages, domains, sender signatures, stats, inbound rules,
//! webhooks, message stream suppressions, and data removals.
//!
//! # Example:
//! ```
//! use postmark::reqwest::PostmarkClient;
//! use postmark::*;
//!
//! # async fn send_email(){
//! let client = PostmarkClient::builder()
//!   .base_url("https://api.postmarkapp.com/")
//!   .server_token("<sometoken>")
//!   .build();
//!
//! let req = api::email::SendEmailRequest::builder()
//!   .from("me@example.com")
//!   .to("you@example.com")
//!   .body(api::Body::text("Hi, this is me!".to_string()))
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
