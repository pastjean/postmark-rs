use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::api::signatures::Signature;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct GetSignatureRequest {
    #[serde(skip)]
    pub signature_id: isize,
}

pub type GetSignatureResponse = Signature;

impl Endpoint for GetSignatureRequest {
    type Request = GetSignatureRequest;
    type Response = GetSignatureResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/senders/{}", self.signature_id).into()
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
    async fn get_signature_gets_sender_by_id() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/senders/22")).respond_with(
                json_encoded(json!({
                    "ID": 22,
                    "Name": "Ops",
                    "EmailAddress": "ops@example.com"
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = GetSignatureRequest::builder().signature_id(22).build();

        assert_eq!(req.method(), http::Method::GET);
        assert_eq!(req.endpoint(), "/senders/22");

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode get signature");

        assert_eq!(resp.id, 22);
        assert_eq!(resp.email_address, "ops@example.com");
    }
}
