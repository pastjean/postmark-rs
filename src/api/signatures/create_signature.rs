use std::borrow::Cow;

use crate::api::signatures::SenderSignature;
use crate::Endpoint;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct CreateSignatureRequest {
    #[serde(rename = "FromEmail")]
    pub from_email: String,
    pub name: String,
    #[builder(default, setter(into, strip_option))]
    #[serde(rename = "ReplyToEmail", skip_serializing_if = "Option::is_none")]
    pub reply_to_email: Option<String>,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_path_domain: Option<String>,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_personal_note: Option<String>,
}

impl Endpoint for CreateSignatureRequest {
    type Request = CreateSignatureRequest;
    type Response = SenderSignature;

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

    use crate::reqwest::PostmarkClient;
    use crate::Query;

    use super::*;

    #[tokio::test]
    async fn create_signature() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("POST", "/senders")).respond_with(
                json_encoded(json!({
                    "Domain": "example.com",
                    "EmailAddress": "john@example.com",
                    "ReplyToEmailAddress": "reply@example.com",
                    "Name": "John",
                    "Confirmed": false,
                    "SPFVerified": false,
                    "DKIMVerified": false,
                    "WeakDKIM": false,
                    "ID": 1
                })),
            ),
        );
        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = CreateSignatureRequest::builder()
            .from_email("john@example.com".to_string())
            .name("John".to_string())
            .build();
        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.id, 1);
    }
}
