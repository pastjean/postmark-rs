//! Postmark is an HTTP client-agnostic Rust client for Postmark.
//! It provides a `reqwest` implementation that can be initialized and passed
//! into the execute function of a [`Query`]. All [`Endpoint`] types implement
//! the Query trait.
//!
//! Some endpoints are already provided. If you need one that is not
//! implemented, you are not constrained to modify this crate. You can
//! implement your own by implementing the [`Endpoint`] trait and it will
//! work transparently with this library.
//!
//! To use the [`reqwest`] based client, enable the `"reqwest"` feature.
//! You can also implement your own client by implementing the [`Client`] trait.
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
