use std::borrow::Cow;

use crate::Endpoint;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteWebhookRequest {
    #[serde(skip)]
    pub id: isize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteWebhookResponse {
    pub error_code: isize,
    pub message: String,
}

impl Endpoint for DeleteWebhookRequest {
    type Request = DeleteWebhookRequest;
    type Response = DeleteWebhookResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/webhooks/{}", self.id).into()
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
    use httptest::{Expectation, Server, responders::*};
    use serde_json::json;

    use crate::Query;
    use crate::reqwest::PostmarkClient;

    use super::*;

    #[tokio::test]
    pub async fn delete_webhook() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("DELETE", "/webhooks/1234")).respond_with(
                json_encoded(json!({
                    "ErrorCode": 0,
                    "Message": "Webhook 1234 removed."
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = DeleteWebhookRequest::builder().id(1234).build();

        let resp = req
            .execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");

        assert_eq!(resp.error_code, 0);
        assert_eq!(resp.message, "Webhook 1234 removed.");
    }
}
