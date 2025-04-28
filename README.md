# Postmark

[![ci](https://github.com/pastjean/postmark-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/pastjean/postmark-rs/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/postmark.svg)](https://crates.io/crates/postmark)
[![Documentation](https://docs.rs/postmark/badge.svg)](https://docs.rs/postmark)
[![License](https://img.shields.io/crates/l/postmark.svg)](#license)

A rust library to query Postmark API.

# Usage

Add the crate dependency to your Cargo.toml:

```toml
[dependencies]
postmark = "x.y.z"
```

And use it, see documentation at: https://docs.rs/postmark.

```rust
use postmark::api::email::SendEmailRequest;
use postmark::api::Body;
use postmark::reqwest::PostmarkClient;
use postmark::Query;

async fn send_email(){
  let client = PostmarkClient::builder()
   .server_token("<sometoken>")
   .build();

  let req = SendEmailRequest::builder()
    .from("me@example.com")
    .to("you@example.com")
    .body(Body::text("it's me, Mario!".to_string()))
    .build();
  let resp = req.execute(&client).await;
}
```

# Releasing a new version

Prerequisite:

```sh
cargo install cargo-release
```

On Release:

```sh
cargo release --dry-run
# check it does the good thing
cargo release
```

# Thanks

This crate's API design is heavily inspired by the article ["Designing Rust bindings for REST APIs](https://plume.benboeckel.net/~/JustAnotherBlog/designing-rust-bindings-for-rest-ap-is) by Ben Boeckel.

# License

postmark-rs is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.
