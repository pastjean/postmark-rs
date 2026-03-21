use std::borrow::Cow;

use crate::Endpoint;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct GetBulkStatusRequest {
    #[serde(skip)]
    pub bulk_request_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetBulkStatusResponse {
    #[serde(alias = "Id", rename = "ID")]
    pub id: String,
    pub submitted_at: String,
    pub total_messages: isize,
    pub percentage_completed: f64,
    pub status: String,
    pub subject: Option<String>,
}

impl Endpoint for GetBulkStatusRequest {
    type Request = GetBulkStatusRequest;
    type Response = GetBulkStatusResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/email/bulk/{}", self.bulk_request_id).into()
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
    use httptest::{responders::*, Expectation, Server};
    use serde_json::json;

    use crate::reqwest::PostmarkClient;
    use crate::Query;

    use super::*;

    #[tokio::test]
    async fn get_bulk_status() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path(
                "GET",
                "/email/bulk/dc5e5d98-c073-4c97-8ee5-f897dfd28b47",
            ))
            .respond_with(json_encoded(json!({
                "Id": "dc5e5d98-c073-4c97-8ee5-f897dfd28b47",
                "SubmittedAt": "2024-07-22T15:39:49.3723691Z",
                "TotalMessages": 1,
                "PercentageCompleted": 1,
                "Status": "Completed",
                "Subject": "Hello"
            }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = GetBulkStatusRequest::builder()
            .bulk_request_id("dc5e5d98-c073-4c97-8ee5-f897dfd28b47".to_string())
            .build();
        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.status, "Completed");
    }
}
