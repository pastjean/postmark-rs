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
        let method = self.method();
        let mut req_builder = http::Request::builder()
            .method(method.clone())
            .uri(String::from(self.endpoint()))
            .header("Accept", "application/json");

        let body = match method {
            http::Method::GET | http::Method::DELETE | http::Method::HEAD => Bytes::new(),
            _ => {
                req_builder = req_builder.header("Content-Type", "application/json");
                serde_json::to_vec(self.body())?.into()
            }
        };

        let http_req = req_builder.body(body)?;

        let response = client.execute(http_req).await.map_err(QueryError::client)?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use http::StatusCode;
    use std::borrow::Cow;
    use std::sync::{Arc, Mutex};

    #[derive(Debug, thiserror::Error)]
    #[error("test client error")]
    struct TestClientError;

    #[derive(Clone)]
    struct TestClient {
        last_request: Arc<Mutex<Option<Request<Bytes>>>>,
    }

    impl TestClient {
        fn new() -> Self {
            Self {
                last_request: Arc::new(Mutex::new(None)),
            }
        }

        fn last_request(&self) -> Request<Bytes> {
            self.last_request
                .lock()
                .expect("lock")
                .clone()
                .expect("request present")
        }
    }

    #[async_trait]
    impl Client for TestClient {
        type Error = TestClientError;

        async fn execute(&self, req: Request<Bytes>) -> Result<Response<Bytes>, Self::Error> {
            *self.last_request.lock().expect("lock") = Some(req);
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(Bytes::from_static(br#"{"ok":true}"#))
                .expect("response"))
        }
    }

    #[derive(serde::Serialize)]
    struct NoBody;

    #[derive(serde::Serialize)]
    struct SomeBody {
        value: &'static str,
    }

    #[derive(serde::Deserialize)]
    struct OkResponse {
        ok: bool,
    }

    struct GetEndpoint;
    impl Endpoint for GetEndpoint {
        type Request = NoBody;
        type Response = OkResponse;

        fn endpoint(&self) -> Cow<'static, str> {
            "/test-get".into()
        }

        fn body(&self) -> &Self::Request {
            static BODY: NoBody = NoBody;
            &BODY
        }

        fn method(&self) -> http::Method {
            http::Method::GET
        }
    }

    struct DeleteEndpoint;
    impl Endpoint for DeleteEndpoint {
        type Request = NoBody;
        type Response = OkResponse;

        fn endpoint(&self) -> Cow<'static, str> {
            "/test-delete".into()
        }

        fn body(&self) -> &Self::Request {
            static BODY: NoBody = NoBody;
            &BODY
        }

        fn method(&self) -> http::Method {
            http::Method::DELETE
        }
    }

    struct PostEndpoint;
    impl Endpoint for PostEndpoint {
        type Request = SomeBody;
        type Response = OkResponse;

        fn endpoint(&self) -> Cow<'static, str> {
            "/test-post".into()
        }

        fn body(&self) -> &Self::Request {
            static BODY: SomeBody = SomeBody { value: "hello" };
            &BODY
        }
    }

    #[tokio::test]
    async fn get_request_has_no_json_body_or_content_type() {
        let client = TestClient::new();
        let response = GetEndpoint.execute(&client).await.expect("execute");

        assert!(response.ok);

        let request = client.last_request();
        assert_eq!(request.method(), http::Method::GET);
        assert!(request.body().is_empty());
        assert!(request.headers().get("Content-Type").is_none());
        assert_eq!(
            request
                .headers()
                .get("Accept")
                .expect("accept header")
                .to_str()
                .expect("header str"),
            "application/json"
        );
    }

    #[tokio::test]
    async fn delete_request_has_no_json_body_or_content_type() {
        let client = TestClient::new();
        let response = DeleteEndpoint.execute(&client).await.expect("execute");

        assert!(response.ok);

        let request = client.last_request();
        assert_eq!(request.method(), http::Method::DELETE);
        assert!(request.body().is_empty());
        assert!(request.headers().get("Content-Type").is_none());
    }

    #[tokio::test]
    async fn post_request_keeps_json_body_and_content_type() {
        let client = TestClient::new();
        let response = PostEndpoint.execute(&client).await.expect("execute");

        assert!(response.ok);

        let request = client.last_request();
        assert_eq!(request.method(), http::Method::POST);
        assert_eq!(request.body(), &Bytes::from_static(br#"{"value":"hello"}"#));
        assert_eq!(
            request
                .headers()
                .get("Content-Type")
                .expect("content type")
                .to_str()
                .expect("header str"),
            "application/json"
        );
    }
}
