use std::borrow::Cow;

use crate::Endpoint;
use crate::api::signatures::{SenderSignatureSummary, paginated_endpoint};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct ListSignaturesRequest {
    #[serde(skip)]
    pub count: i64,
    #[serde(skip)]
    pub offset: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListSignaturesResponse {
    pub total_count: i64,
    pub sender_signatures: Vec<SenderSignatureSummary>,
}

impl Endpoint for ListSignaturesRequest {
    type Request = ListSignaturesRequest;
    type Response = ListSignaturesResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        paginated_endpoint("/senders", self.count, self.offset)
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
    async fn list_signatures() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/senders")).respond_with(
                json_encoded(json!({
                  "TotalCount": 1,
                  "SenderSignatures": [{
                    "Domain": "example.com",
                    "EmailAddress": "john@example.com",
                    "ReplyToEmailAddress": "reply@example.com",
                    "Name": "John",
                    "Confirmed": true,
                    "ID": 1
                  }]
                })),
            ),
        );
        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = ListSignaturesRequest::builder().count(50).offset(0).build();
        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.total_count, 1);
    }
}
