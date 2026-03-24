use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::Endpoint;
use crate::api::signatures::SenderSignature;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct VerifySignatureSpfRequest {
    #[serde(skip)]
    pub signature_id: isize,
}

pub type VerifySignatureSpfResponse = SenderSignature;

impl Endpoint for VerifySignatureSpfRequest {
    type Request = ();
    type Response = VerifySignatureSpfResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/senders/{}/verifyspf", self.signature_id).into()
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
    use httptest::{Expectation, Server, responders::*};
    use serde_json::json;

    use super::*;
    use crate::Query;
    use crate::reqwest::PostmarkClient;

    #[tokio::test]
    async fn verify_signature_spf_posts_verify_spf_path() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("POST", "/senders/22/verifyspf"))
                .respond_with(json_encoded(json!({
                    "ID": 22,
                    "Domain": "example.com",
                    "Name": "Ops",
                    "EmailAddress": "ops@example.com",
                    "Confirmed": true,
                    "DKIMVerified": false,
                    "WeakDKIM": false
                }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = VerifySignatureSpfRequest::builder()
            .signature_id(22)
            .build();

        assert_eq!(req.method(), http::Method::POST);
        assert_eq!(req.endpoint(), "/senders/22/verifyspf");

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode verify signature spf");

        assert_eq!(resp.id, 22);
    }
}
