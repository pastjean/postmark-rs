use std::borrow::Cow;

use crate::api::signatures::SenderSignature;
use crate::Endpoint;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct EditSignatureRequest {
    #[serde(skip)]
    pub signature_id: isize,
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

impl Endpoint for EditSignatureRequest {
    type Request = EditSignatureRequest;
    type Response = SenderSignature;

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

    use crate::reqwest::PostmarkClient;
    use crate::Query;

    use super::*;

    #[tokio::test]
    async fn edit_signature() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("PUT", "/senders/1")).respond_with(
                json_encoded(json!({
                    "Domain": "example.com",
                    "EmailAddress": "john@example.com",
                    "ReplyToEmailAddress": "jane@example.com",
                    "Name": "Jane Doe",
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
        let req = EditSignatureRequest::builder()
            .signature_id(1)
            .name("Jane Doe".to_string())
            .reply_to_email("jane@example.com".to_string())
            .build();
        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.name, "Jane Doe");
    }
}
