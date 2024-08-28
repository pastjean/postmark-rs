use std::borrow::Cow;

use async_trait::async_trait;
use bytes::Bytes;
use http::{Request, Response};
use std::error::Error;
use thiserror::Error;

/// A trait for providing the necessary information for a single REST API endpoint.
pub trait Endpoint {
    type Request: serde::Serialize + Send + Sync;
    type Response: serde::de::DeserializeOwned + Send + Sync;

    /// The path to the endpoint.
    fn endpoint(&self) -> Cow<'static, str>;
    /// The body for the endpoint.
    fn body(&self) -> &Self::Request;
    /// The http method for the endpoint
    fn method(&self) -> http::Method {
        http::Method::POST
    }
}

/// A trait which represents an asynchronous query which may be made to a Postmark client.
#[async_trait]
pub trait Query<C> {
    /// The Result of executing a query
    type Result;
    /// Perform the query against the client.
    async fn execute(self, client: &C) -> Self::Result;
}

/// An error thrown by the [`Query`] trait
#[derive(Debug, Error)]
pub enum QueryError<E>
where
    E: Error + Send + Sync + 'static,
{
    /// The client encountered an error.
    #[error("client error: {}", source)]
    Client {
        /// The client error.
        source: E,
    },
    /// JSON deserialization from GitLab failed.
    #[error("could not parse JSON response: {}", source)]
    Json {
        /// The source of the error.
        #[from]
        source: serde_json::Error,
    },
    /// Body data could not be created.
    #[error("failed to create form data: {}", source)]
    Body {
        /// The source of the error.
        #[from]
        source: http::Error,
    },
}

impl<E> QueryError<E>
where
    E: Error + Send + Sync + 'static,
{
    /// Create an API error in a client error.
    pub fn client(source: E) -> Self {
        QueryError::Client { source }
    }
}

/// Extension method to all Endpoints to execute themselves againts a query
#[async_trait]
impl<T, C> Query<C> for T
where
    T: Endpoint + Send + Sync,
    C: Client + Send + Sync,
{
    /// An endpoint return it's Response or the Client's Error
    type Result = Result<T::Response, QueryError<C::Error>>;

    async fn execute(self, client: &C) -> Self::Result {
        let body = serde_json::to_vec(self.body())?;

        let http_req = http::Request::builder()
            .method(self.method())
            .uri(String::from(self.endpoint()))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .body(body.into())?;

        let response = client
            .execute(http_req.clone())
            .await
            .map_err(QueryError::client)?;

        Ok(serde_json::from_slice(response.body())?)
    }
}

/// A trait representing a client which can communicate with a Postmark instance.
#[async_trait]
pub trait Client {
    /// The errors which may occur for this client.
    type Error: Error + Send + Sync + 'static;
    /// Execute the request which was formed by [`Endpoint`]
    async fn execute(&self, req: Request<Bytes>) -> Result<Response<Bytes>, Self::Error>;
}
