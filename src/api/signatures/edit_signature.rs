use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::api::signatures::Signature;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct EditSignatureRequest {
    #[serde(skip)]
    pub signature_id: isize,
    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(into))]
    pub email_address: String,
}

pub type EditSignatureResponse = Signature;

impl Endpoint for EditSignatureRequest {
    type Request = EditSignatureRequest;
    type Response = EditSignatureResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/senders/{}", self.signature_id).into()
    }

    fn body(&self) -> &Self::Request {
        self
    }

    fn method(&self) -> http::Method {
        http::Method::PUT
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
    async fn edit_signature_puts_sender() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("PUT", "/senders/22")).respond_with(
                json_encoded(json!({
                    "ID": 22,
                    "Name": "Ops Team",
                    "EmailAddress": "ops@example.com"
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = EditSignatureRequest::builder()
            .signature_id(22)
            .name("Ops Team")
            .email_address("ops@example.com")
            .build();

        assert_eq!(req.method(), http::Method::PUT);
        assert_eq!(req.endpoint(), "/senders/22");
        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            json!({ "Name": "Ops Team", "EmailAddress": "ops@example.com" })
        );

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode edit signature");

        assert_eq!(resp.name, "Ops Team");
    }
}
