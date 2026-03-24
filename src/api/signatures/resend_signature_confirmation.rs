use std::borrow::Cow;

use crate::Endpoint;
use crate::api::signatures::{BasicApiResponse, SignatureId};
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct ResendSignatureConfirmationRequest {
    #[builder(setter(into))]
    #[serde(skip)]
    pub signature_id: SignatureId,
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
    use httptest::{Expectation, Server, responders::*};
    use serde_json::json;

    use crate::Query;
    use crate::reqwest::PostmarkClient;

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
