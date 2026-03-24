use std::borrow::Cow;

use serde::Serialize;

use crate::api::signatures::ListSignaturesResponse;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct ListSignaturesRequest;

impl Endpoint for ListSignaturesRequest {
    type Request = ListSignaturesRequest;
    type Response = ListSignaturesResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "/senders".into()
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

    use super::*;
    use crate::reqwest::PostmarkClient;
    use crate::Query;

    #[tokio::test]
    async fn list_signatures_gets_senders() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/senders")).respond_with(
                json_encoded(json!({
                    "TotalCount": 1,
                    "Senders": [
                        {
                            "ID": 22,
                            "Name": "Ops",
                            "EmailAddress": "ops@example.com"
                        }
                    ]
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = ListSignaturesRequest {};

        assert_eq!(req.method(), http::Method::GET);
        assert_eq!(req.endpoint(), "/senders");

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode list signatures");

        assert_eq!(resp.total_count, 1);
        assert_eq!(resp.signatures[0].id, 22);
    }
}
