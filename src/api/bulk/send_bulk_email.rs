use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::api::email::SendEmailRequest;
use crate::Endpoint;

/// Send bulk emails in async batch mode.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct SendBulkEmailRequest {
    pub messages: Vec<SendEmailRequest>,
}

/// Response for [`SendBulkEmailRequest`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct SendBulkEmailResponse {
    #[serde(rename = "BatchID")]
    pub batch_id: String,
    pub error_code: i64,
    pub message: String,
}

impl SendBulkEmailResponse {
    pub fn error_for_status(self) -> Result<Self, SendBulkEmailResponse> {
        if self.error_code == 0 {
            Ok(self)
        } else {
            Err(self)
        }
    }
}

impl Endpoint for SendBulkEmailRequest {
    type Request = SendBulkEmailRequest;
    type Response = SendBulkEmailResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "/email/bulk".into()
    }

    fn body(&self) -> &Self::Request {
        self
    }
}

#[cfg(test)]
mod tests {
    use httptest::matchers::request;
    use httptest::{responders::*, Expectation, Server};
    use serde_json::json;

    use super::*;
    use crate::api::email::SendEmailRequest;
    use crate::api::Body;
    use crate::reqwest::PostmarkClient;
    use crate::Query;

    fn email(to: &str) -> SendEmailRequest {
        SendEmailRequest::builder()
            .from("pa@example.com")
            .to(to)
            .body(Body::text("hello".to_string()))
            .subject("subject")
            .build()
    }

    #[tokio::test]
    async fn send_bulk_email_posts_to_bulk_endpoint() {
        let server = Server::run();
        let mock_response = json!({
            "BatchID": "batch-123",
            "ErrorCode": 0,
            "Message": "OK"
        });

        let deserialized: SendBulkEmailResponse = serde_json::from_value(mock_response.clone())
            .expect("Should deserialize mocked response");
        assert_eq!(deserialized.batch_id, "batch-123");
        assert_eq!(deserialized.error_code, 0);
        assert_eq!(deserialized.message, "OK");

        server.expect(
            Expectation::matching(request::method_path("POST", "/email/bulk"))
                .respond_with(json_encoded(mock_response)),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = SendBulkEmailRequest {
            messages: vec![email("one@example.com"), email("two@example.com")],
        };

        let resp = req.execute(&client).await.expect("Should decode response");
        let resp = resp
            .error_for_status()
            .expect("Should return success status");
        assert_eq!(resp.batch_id, "batch-123");
        assert_eq!(resp.error_code, 0);
        assert_eq!(resp.message, "OK");
    }
}
