use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::Endpoint;

/// Get bulk email batch status and summary counts.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetBulkEmailBatchRequest {
    #[serde(skip)]
    pub batch_id: String,
}

impl GetBulkEmailBatchRequest {
    pub fn new(batch_id: impl Into<String>) -> Self {
        Self {
            batch_id: batch_id.into(),
        }
    }
}

/// Response for [`GetBulkEmailBatchRequest`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct GetBulkEmailBatchResponse {
    #[serde(rename = "BatchID")]
    pub batch_id: String,
    pub status: String,
    pub submitted_at: Option<String>,
    pub completed_at: Option<String>,
    pub total_sent: Option<i64>,
    pub total_failed: Option<i64>,
}

impl Endpoint for GetBulkEmailBatchRequest {
    type Request = ();
    type Response = GetBulkEmailBatchResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/email/bulk/{}", self.batch_id).into()
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
    async fn get_bulk_email_batch_uses_batch_id_path() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/email/bulk/batch-123"))
                .respond_with(json_encoded(json!({
                    "BatchID": "batch-123",
                    "Status": "Completed",
                    "SubmittedAt": "2014-02-17T07:25:01.4178645-05:00",
                    "CompletedAt": "2014-02-17T07:25:21.4178645-05:00",
                    "TotalSent": 2,
                    "TotalFailed": 0
                }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = GetBulkEmailBatchRequest::new("batch-123");
        let resp = req.execute(&client).await.expect("Should decode response");

        assert_eq!(resp.batch_id, "batch-123");
        assert_eq!(resp.status, "Completed");
        assert_eq!(resp.total_sent, Some(2));
        assert_eq!(resp.total_failed, Some(0));
    }
}
