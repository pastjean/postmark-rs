use crate::Endpoint;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetBounceResponse {
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "Type")]
    pub type_field: Option<String>,
    pub email: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetBounceRequest {
    #[serde(skip)]
    pub bounce_id: i64,
}

impl GetBounceRequest {
    pub fn new(bounce_id: i64) -> Self {
        Self { bounce_id }
    }
}

impl Endpoint for GetBounceRequest {
    type Request = ();
    type Response = GetBounceResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/bounces/{}", self.bounce_id).into()
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
    async fn send_get_bounce() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/bounces/777")).respond_with(
                json_encoded(json!({
                    "ID": 777,
                    "Type": "HardBounce",
                    "Email": "bounce@example.com"
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = GetBounceRequest::new(777);

        let resp = req
            .execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");

        assert_eq!(resp.id, 777);
        assert_eq!(resp.type_field, Some(String::from("HardBounce")));
        assert_eq!(resp.email, "bounce@example.com");
    }
}
