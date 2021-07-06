use std::borrow::Cow;

use async_trait::async_trait;

/// A trait for providing the necessary information for a single REST API endpoint.
/// Ths
pub trait Endpoint {
    type Request: serde::Serialize + Send + Sync;
    type Response: serde::de::DeserializeOwned + Send + Sync;

    /// The path to the endpoint.
    fn endpoint(&self) -> Cow<'static, str>;
    /// The body for the endpoint.
    fn body(&self) -> &Self::Request;
}

/// A trait which represents an asynchronous query which may be made to a Postmark client.
#[async_trait]
pub trait Query<C> {
    /// The Result of executing a query
    type Result;

    /// Perform the query against the client.
    async fn execute(self, client: &C) -> Self::Result;
}

/// Extension method to all Endpoints to execute themselves againts a query
#[async_trait]
impl<T, C> Query<C> for T
where
    T: Endpoint + Send + Sync,
    C: Client + Send + Sync,
{
    /// An endpoint return it's Response or the Client's Error
    type Result = Result<T::Response, C::Error>;

    async fn execute(self, client: &C) -> Self::Result {
        client.execute(self).await
    }
}

/// A trait representing a client which can communicate with a Postmark instance.
#[async_trait]
pub trait Client {
    /// The errors which may occur for this client.
    type Error;
    /// Execute the request which was formed by [`Endpoint`]
    async fn execute<R>(&self, req: R) -> Result<R::Response, Self::Error>
    where
        R: Endpoint + Send + Sync;
}
