use std::borrow::Cow;

use crate::Endpoint;
use crate::api::server::Server;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct GetCurrentServerRequest {}

impl Endpoint for GetCurrentServerRequest {
    type Request = GetCurrentServerRequest;
    type Response = Server;

    fn endpoint(&self) -> Cow<'static, str> {
        "/server".into()
    }

    fn body(&self) -> &Self::Request {
        self
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
}

#[cfg(test)]
mod tests {
    use httptest::matchers::request;
    use httptest::{Expectation, Server as HttpServer, responders::*};
    use serde_json::json;

    use crate::Query;
    use crate::reqwest::PostmarkClient;

    use super::*;

    #[tokio::test]
    pub async fn get_current_server() {
        let server = HttpServer::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/server")).respond_with(
                json_encoded(json!({
                  "ID": 1,
                  "Name": "Staging Testing",
                  "ApiTokens": ["server token"],
                  "SmtpApiActivated": true
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = GetCurrentServerRequest::builder().build();

        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.id, 1);
        assert_eq!(resp.name, "Staging Testing");
    }
}
