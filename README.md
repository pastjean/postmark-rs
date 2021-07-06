# Postmark

[![CI](https://github.com/braverhealth/postmark-rust/workflows/ci/badge.svg)](https://github.com/braverhealth/postmark-rust/actions?query=workflow%3Aci)
[![crates.io](https://img.shields.io/crates/v/postmark.svg)](https://crates.io/crates/postmark)
[![Documentation](https://docs.rs/postmark/badge.svg)](https://docs.rs/postmark)
[![License](https://img.shields.io/crates/l/postmark.svg)](https://github.com/braverhealth/posrmark-rust#license)

A rust library to query Postmark API.

# Usage

Add the crate dependency to your Cargo.toml:

```toml
[dependencies]
postmark = "x.y.z"
```

And use it, see documentation at: https://docs.rs/postmark.

```rust
use postmark::reqwest::PostmarkClient;
use postmark::*;

async fn send_email(){
  let client = PostmarkClient::builder()
   .base_url("https://api.postmarkapp.com/")
   .token("<sometoken>")
   .build();

  let req = api::email::SendEmailRequest::builder()
    .from("me@example.com")
    .to("you@example.com")
    .body(api::email::Body::Text("Hi, this is me!".to_string()))
    .build();
  let resp = req.execute(&client).await.unwrap();
}
```

# Thanks

This crate is heavily inspired by the article ["Designing Rust bindings for REST APIs](https://plume.benboeckel.net/~/JustAnotherBlog/designing-rust-bindings-for-rest-ap-is) by Ben Boeckel.

# License

postmark is distributed under the terms of both the MIT license and the Apache License (Version 2.0).
