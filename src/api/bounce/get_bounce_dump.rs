use std::borrow::Cow;

use crate::Endpoint;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct GetBounceDumpRequest {
    #[serde(skip)]
    pub bounce_id: isize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetBounceDumpResponse {
    pub body: String,
}

impl Endpoint for GetBounceDumpRequest {
    type Request = GetBounceDumpRequest;
    type Response = GetBounceDumpResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/bounces/{}/dump", self.bounce_id).into()
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
    use httptest::{Expectation, Server, responders::*};
    use serde_json::json;

    use crate::Query;
    use crate::reqwest::PostmarkClient;

    use super::*;

    #[tokio::test]
    pub async fn get_bounce_dump() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/bounces/42/dump")).respond_with(
                json_encoded(json!({
                    "Body": "raw mime body"
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = GetBounceDumpRequest::builder().bounce_id(42).build();
        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.body, "raw mime body");
    }
}
