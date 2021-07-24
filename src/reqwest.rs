use std::convert::TryInto;

use crate::{Client, POSTMARK_API_URL};
use async_trait::async_trait;
use bytes::Bytes;
use http::{Request, Response};
use thiserror::Error;
use typed_builder::TypedBuilder;

/// A representation of the asynchronous Postmark API for a single user.
/// Separate users should use separate instances of this.
///
/// A reqwest based [`Client`]
///
/// ```
/// # use postmark::reqwest::PostmarkClient;
/// let client = PostmarkClient::default();
/// ```
///
/// Using the builder:
/// ```
/// # use postmark::reqwest::PostmarkClient;
/// let client = PostmarkClient::builder()
///   .base_url("https://api.postmarkapp.com/")
///   .token("<sometoken>")
///   .build();
/// ```
#[derive(TypedBuilder)]
pub struct PostmarkClient {
    #[builder(default, setter(into, strip_option))]
    pub token: Option<String>,
    #[builder(default=POSTMARK_API_URL.into(), setter(into))]
    pub base_url: String,
}

impl std::fmt::Debug for PostmarkClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            token: ref _token,
            base_url: ref _base_url,
        } = *self;

        let mut builder = f.debug_struct("PostmarkClient");
        builder.field("token", &_token.as_ref().map(|_| "***"));
        builder.field("base_url", &(*_base_url));
        builder.finish()
    }
}

impl Default for PostmarkClient {
    fn default() -> Self {
        Self {
            base_url: POSTMARK_API_URL.into(),
            token: None,
        }
    }
}

#[derive(Error, Debug)]
pub enum PostmarkClientError {
    #[error("error setting auth header: {}", source)]
    AuthError {
        #[from]
        source: http::header::InvalidHeaderValue,
    },
    #[error("communication with postmark: {}", source)]
    Communication {
        #[from]
        source: reqwest::Error,
    },
    #[error("`http` error: {}", source)]
    Http {
        #[from]
        source: http::Error,
    },
}

#[async_trait]
impl Client for PostmarkClient {
    type Error = PostmarkClientError;

    async fn execute(&self, req: Request<Bytes>) -> Result<Response<Bytes>, Self::Error> {
        let client = reqwest::Client::builder().build()?;
        let mut req = req;

        if let Some(tok) = &self.token {
            req.headers_mut()
                .append("X-Postmark-Server-Token", tok.try_into()?);
        }

        let reqwest_req = req.try_into()?;
        let reqwest_rsp = client.execute(reqwest_req).await?;

        let mut rsp = Response::builder()
            .status(reqwest_rsp.status())
            .version(reqwest_rsp.version());

        let headers = rsp.headers_mut().unwrap();
        for (k, v) in reqwest_rsp.headers() {
            headers.insert(k, v.clone());
        }

        Ok(rsp.body(reqwest_rsp.bytes().await?)?)
    }
}
