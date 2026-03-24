use crate::Endpoint;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetBounceDumpRequest {
    #[serde(skip)]
    pub bounce_id: i64,
}

impl GetBounceDumpRequest {
    pub fn new(bounce_id: i64) -> Self {
        Self { bounce_id }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetBounceDumpResponse {
    pub body: String,
}

impl Endpoint for GetBounceDumpRequest {
    type Request = ();
    type Response = GetBounceDumpResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/bounces/{}/dump", self.bounce_id).into()
    }

    fn body(&self) -> &Self::Request {
        &()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
}

#[cfg(test)]
mod tests {
    use httptest::matchers::request;
    use httptest::{responders::*, Expectation, Server};
    use serde_json::json;

    use super::*;
    use crate::reqwest::PostmarkClient;
    use crate::Query;

    #[tokio::test]
    async fn send_get_bounce_dump() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/bounces/777/dump")).respond_with(
                json_encoded(json!({
                    "Body": "Raw bounce body"
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = GetBounceDumpRequest::new(777);

        let resp = req
            .execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");

        assert_eq!(resp.body, "Raw bounce body");
    }
}
