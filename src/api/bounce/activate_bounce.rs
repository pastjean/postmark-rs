use crate::Endpoint;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::get_bounce::GetBounceResponse;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ActivateBounceRequest {
    #[serde(skip)]
    pub bounce_id: i64,
}

impl ActivateBounceRequest {
    pub fn new(bounce_id: i64) -> Self {
        Self { bounce_id }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActivateBounceResponse {
    pub message: String,
    pub bounce: GetBounceResponse,
}

impl Endpoint for ActivateBounceRequest {
    type Request = ();
    type Response = ActivateBounceResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/bounces/{}/activate", self.bounce_id).into()
    }

    fn body(&self) -> &Self::Request {
        &()
    }

    fn method(&self) -> http::Method {
        http::Method::PUT
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
    async fn send_activate_bounce() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("PUT", "/bounces/777/activate"))
                .respond_with(json_encoded(json!({
                    "Message": "OK",
                    "Bounce": {
                        "ID": 777,
                        "Type": "HardBounce",
                        "Email": "bounce@example.com"
                    }
                }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = ActivateBounceRequest::new(777);

        let resp = req
            .execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");

        assert_eq!(resp.message, "OK");
        assert_eq!(resp.bounce.id, 777);
    }
}
