use std::borrow::Cow;

use crate::Endpoint;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct CreateDataRemovalRequest {
    pub requested_by: String,
    pub requested_for: String,
    pub notify_when_completed: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataRemovalStatusResponse {
    #[serde(rename = "ID")]
    pub id: isize,
    pub status: String,
}

impl Endpoint for CreateDataRemovalRequest {
    type Request = CreateDataRemovalRequest;
    type Response = DataRemovalStatusResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "/data-removals".into()
    }

    fn body(&self) -> &Self::Request {
        self
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
    async fn create_data_removal() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("POST", "/data-removals"))
                .respond_with(json_encoded(json!({"ID": 1234, "Status": "Pending"}))),
        );
        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = CreateDataRemovalRequest::builder()
            .requested_by("a@example.com".to_string())
            .requested_for("b@example.com".to_string())
            .notify_when_completed(true)
            .build();
        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.status, "Pending");
    }
}
