use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::api::server::ServerIdOrName;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteServerRequest {
    #[serde(skip)]
    pub server_id: ServerIdOrName,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteServerResponse {
    pub error_code: i64,
    pub message: String,
}

impl Endpoint for DeleteServerRequest {
    type Request = DeleteServerRequest;
    type Response = DeleteServerResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/servers/{}", self.server_id).into()
    }

    fn body(&self) -> &Self::Request {
        self
    }

    fn method(&self) -> http::Method {
        http::Method::DELETE
    }
}

#[cfg(test)]
mod tests {
    use httptest::matchers::request;
    use httptest::{responders::*, Expectation, Server};
    use serde_json::json;

    use crate::reqwest::PostmarkClient;
    use crate::Query;

    use super::*;

    #[tokio::test]
    async fn delete_server_deletes_by_id() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("DELETE", "/servers/12345")).respond_with(
                json_encoded(json!({
                    "ErrorCode": 0,
                    "Message": "OK"
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = DeleteServerRequest::builder()
            .server_id(ServerIdOrName::ServerId(12345))
            .build();

        assert_eq!(req.method(), http::Method::DELETE);
        assert_eq!(req.endpoint(), "/servers/12345");

        req.execute(&client)
            .await
            .expect("Should decode delete server response");
    }

    #[test]
    fn delete_server_endpoint_supports_server_name() {
        let req = DeleteServerRequest::builder()
            .server_id(ServerIdOrName::ServerName("staging".into()))
            .build();

        assert_eq!(req.endpoint(), "/servers/staging");
    }
}
