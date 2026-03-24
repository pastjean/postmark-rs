use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::api::signatures::Signature;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct ResendSignatureConfirmationRequest {
    #[serde(skip)]
    pub signature_id: isize,
}

pub type ResendConfirmationResponse = Signature;

impl Endpoint for ResendSignatureConfirmationRequest {
    type Request = ();
    type Response = ResendConfirmationResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/senders/{}/resend", self.signature_id).into()
    }

    fn body(&self) -> &Self::Request {
        &()
    }

    fn method(&self) -> http::Method {
        http::Method::POST
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
    async fn resend_confirmation_posts_resend_path() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("POST", "/senders/22/resend")).respond_with(
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

        let req = ResendSignatureConfirmationRequest::builder()
            .signature_id(22)
            .build();

        assert_eq!(req.method(), http::Method::POST);
        assert_eq!(req.endpoint(), "/senders/22/resend");

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode resend confirmation");

        assert_eq!(resp.id, 22);
    }
}
