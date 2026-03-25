use std::convert::TryInto;

use crate::{Client, POSTMARK_API_URL};
use crate::{Endpoint, Query, QueryError};
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
///
/// Endpoint facade example:
/// ```
/// # use postmark::reqwest::PostmarkClient;
/// # use postmark::api::messages::OutboundSearchRequest;
/// # async fn demo() {
/// let client = PostmarkClient::builder()
///   .server_token("<sometoken>")
///   .build();
/// let request = OutboundSearchRequest::builder().build();
/// let _response = client.messages().outbound_search(request).await;
/// # }
/// ```
#[derive(TypedBuilder, Clone)]
pub struct PostmarkClient {
    #[builder(default, setter(into, strip_option))]
    pub server_token: Option<String>,
    #[builder(default, setter(into, strip_option))]
    pub account_token: Option<String>,
    #[builder(default=POSTMARK_API_URL.into(), setter(into))]
    pub base_url: String,
    #[builder(default=::reqwest::Client::new(), setter(skip))]
    client: reqwest::Client,
}

impl std::fmt::Debug for PostmarkClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            server_token: ref _server_token,
            account_token: ref _account_token,
            base_url: ref _base_url,
            client: ref _client,
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
            client: reqwest::Client::new(),
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
    #[error("invalid uri: {}", source)]
    InvalidUri {
        #[from]
        source: http::uri::InvalidUri,
    },
}

#[async_trait]
impl Client for PostmarkClient {
    type Error = PostmarkClientError;

    async fn execute(&self, req: Request<Bytes>) -> Result<Response<Bytes>, Self::Error> {
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

        *req.uri_mut() = url.as_str().parse()?;

        let reqwest_req: reqwest::Request = req.try_into()?;
        let reqwest_rsp = self.client.execute(reqwest_req).await?;

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

impl PostmarkClient {
    pub async fn execute_endpoint<T>(
        &self,
        request: T,
    ) -> Result<T::Response, QueryError<PostmarkClientError>>
    where
        T: Endpoint + Send + Sync,
    {
        request.execute(self).await
    }

    pub fn messages(&self) -> MessagesApi<'_> {
        MessagesApi { client: self }
    }
}

pub struct MessagesApi<'a> {
    client: &'a PostmarkClient,
}

impl MessagesApi<'_> {
    pub async fn outbound_search(
        &self,
        request: crate::api::messages::OutboundSearchRequest,
    ) -> Result<crate::api::messages::OutboundSearchResponse, QueryError<PostmarkClientError>> {
        self.client.execute_endpoint(request).await
    }

    pub async fn outbound_details(
        &self,
        request: crate::api::messages::OutboundDetailsRequest,
    ) -> Result<crate::api::messages::OutboundDetailsResponse, QueryError<PostmarkClientError>>
    {
        self.client.execute_endpoint(request).await
    }

    pub async fn outbound_dump(
        &self,
        request: crate::api::messages::OutboundDumpRequest,
    ) -> Result<crate::api::messages::OutboundDumpResponse, QueryError<PostmarkClientError>> {
        self.client.execute_endpoint(request).await
    }

    pub async fn inbound_search(
        &self,
        request: crate::api::messages::InboundSearchRequest,
    ) -> Result<crate::api::messages::InboundSearchResponse, QueryError<PostmarkClientError>> {
        self.client.execute_endpoint(request).await
    }

    pub async fn inbound_details(
        &self,
        request: crate::api::messages::InboundDetailsRequest,
    ) -> Result<crate::api::messages::InboundDetailsResponse, QueryError<PostmarkClientError>> {
        self.client.execute_endpoint(request).await
    }

    pub async fn message_opens(
        &self,
        request: crate::api::messages::MessageOpensRequest,
    ) -> Result<crate::api::messages::MessageOpensResponse, QueryError<PostmarkClientError>> {
        self.client.execute_endpoint(request).await
    }

    pub async fn single_message_opens(
        &self,
        request: crate::api::messages::SingleMessageOpensRequest,
    ) -> Result<crate::api::messages::MessageOpensResponse, QueryError<PostmarkClientError>> {
        self.client.execute_endpoint(request).await
    }

    pub async fn message_clicks(
        &self,
        request: crate::api::messages::MessageClicksRequest,
    ) -> Result<crate::api::messages::MessageClicksResponse, QueryError<PostmarkClientError>> {
        self.client.execute_endpoint(request).await
    }

    pub async fn single_message_clicks(
        &self,
        request: crate::api::messages::SingleMessageClicksRequest,
    ) -> Result<crate::api::messages::MessageClicksResponse, QueryError<PostmarkClientError>> {
        self.client.execute_endpoint(request).await
    }

    pub async fn bypass_blocked_inbound(
        &self,
        request: crate::api::messages::BypassBlockedInboundRequest,
    ) -> Result<crate::api::messages::BypassBlockedInboundResponse, QueryError<PostmarkClientError>>
    {
        self.client.execute_endpoint(request).await
    }

    pub async fn retry_failed_inbound(
        &self,
        request: crate::api::messages::RetryFailedInboundRequest,
    ) -> Result<crate::api::messages::RetryFailedInboundResponse, QueryError<PostmarkClientError>>
    {
        self.client.execute_endpoint(request).await
    }
}

#[cfg(test)]
mod tests {
    use httptest::matchers::request;
    use httptest::{Expectation, Server, responders::*};
    use serde_json::json;

    use crate::api::messages::OutboundSearchRequest;

    use super::*;

    #[tokio::test]
    async fn messages_facade_executes_outbound_search() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/messages/outbound")).respond_with(
                json_encoded(json!({
                    "TotalCount": 1,
                    "Messages": [{"MessageID": "out-msg-1"}]
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let request = OutboundSearchRequest::builder().build();
        let response = client
            .messages()
            .outbound_search(request)
            .await
            .expect("outbound search through facade");

        assert_eq!(response.total_count, 1);
    }
}
