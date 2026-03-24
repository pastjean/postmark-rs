use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::api::signatures::Signature;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct CreateSignatureRequest {
    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(into))]
    pub email_address: String,
}

pub type CreateSignatureResponse = Signature;

impl Endpoint for CreateSignatureRequest {
    type Request = CreateSignatureRequest;
    type Response = CreateSignatureResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "/senders".into()
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
    use crate::reqwest::PostmarkClient;
    use crate::Query;

    #[tokio::test]
    async fn create_signature_posts_sender() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("POST", "/senders")).respond_with(
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

        let req = CreateSignatureRequest::builder()
            .name("Ops")
            .email_address("ops@example.com")
            .build();

        assert_eq!(req.method(), http::Method::POST);
        assert_eq!(req.endpoint(), "/senders");
        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            json!({ "Name": "Ops", "EmailAddress": "ops@example.com" })
        );

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode create signature");

        assert_eq!(resp.id, 22);
    }
}
