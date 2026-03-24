use std::borrow::Cow;

use crate::api::signatures::BasicApiResponse;
use crate::Endpoint;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct ResendSignatureConfirmationRequest {
    #[serde(skip)]
    pub signature_id: isize,
}

impl Endpoint for ResendSignatureConfirmationRequest {
    type Request = ResendSignatureConfirmationRequest;
    type Response = BasicApiResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/senders/{}/resend", self.signature_id).into()
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

    use crate::reqwest::PostmarkClient;
    use crate::Query;

    use super::*;

    #[tokio::test]
    async fn resend_signature_confirmation() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("POST", "/senders/1/resend")).respond_with(
                json_encoded(json!({"ErrorCode": 0, "Message": "Confirmation resent."})),
            ),
        );
        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = ResendSignatureConfirmationRequest::builder()
            .signature_id(1)
            .build();
        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.error_code, 0);
    }
}
