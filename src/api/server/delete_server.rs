use std::borrow::Cow;

use crate::Endpoint;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteServerRequest {
    #[serde(skip)]
    pub server_id: isize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteServerResponse {
    pub error_code: isize,
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
    use httptest::{responders::*, Expectation, Server as HttpServer};
    use serde_json::json;

    use crate::reqwest::PostmarkClient;
    use crate::Query;

    use super::*;

    #[tokio::test]
    pub async fn delete_server() {
        let server = HttpServer::run();

        server.expect(
            Expectation::matching(request::method_path("DELETE", "/servers/1")).respond_with(
                json_encoded(json!({
                    "ErrorCode": 0,
                    "Message": "Server 1 removed."
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = DeleteServerRequest::builder().server_id(1).build();

        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.error_code, 0);
    }
}
