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
///   .base_url("https://api.postmarkapp.com")
///   .server_token("<sometoken>")
///   .build();
/// ```
#[derive(TypedBuilder, Clone)]
pub struct PostmarkClient {
    #[builder(default, setter(into, strip_option))]
    pub server_token: Option<String>,
    #[builder(default, setter(into, strip_option))]
    pub account_token: Option<String>,
    #[builder(default=POSTMARK_API_URL.into(), setter(into))]
    pub base_url: String,
}

impl std::fmt::Debug for PostmarkClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            server_token: ref _server_token,
            account_token: ref _account_token,
            base_url: ref _base_url,
        } = *self;

        let mut builder = f.debug_struct("PostmarkClient");
        builder.field("server_token", &_server_token.as_ref().map(|_| "***"));
        builder.field("account_token", &_account_token.as_ref().map(|_| "***"));
        builder.field("base_url", _base_url);
        builder.finish()
    }
}

impl Default for PostmarkClient {
    fn default() -> Self {
        Self {
            base_url: POSTMARK_API_URL.into(),
            server_token: None,
            account_token: None,
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
    #[error("`Url` error: {}", source)]
    UrlParseError {
        #[from]
        source: url::ParseError,
    },
}

#[async_trait]
impl Client for PostmarkClient {
    type Error = PostmarkClientError;

    async fn execute(&self, req: Request<Bytes>) -> Result<Response<Bytes>, Self::Error> {
        let client = reqwest::Client::builder().build()?;
        let mut req = req;

        if let Some(tok) = &self.server_token {
            req.headers_mut()
                .append("X-Postmark-Server-Token", tok.try_into()?);
        }

        if let Some(tok) = &self.account_token {
            req.headers_mut()
                .append("X-Postmark-Account-Token", tok.try_into()?);
        }

        let base_url: url::Url = self.base_url.parse()?;

        let url = match req.uri().path_and_query() {
            Some(path) => base_url.join(path.as_str())?,
            None => base_url,
        };

        *req.uri_mut() = url.as_str().parse().unwrap();

        let reqwest_req: reqwest::Request = req.try_into()?;
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
