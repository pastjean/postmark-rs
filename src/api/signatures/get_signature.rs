use std::borrow::Cow;

use crate::api::signatures::SenderSignature;
use crate::Endpoint;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct GetSignatureRequest {
    #[serde(skip)]
    pub signature_id: isize,
}

impl Endpoint for GetSignatureRequest {
    type Request = GetSignatureRequest;
    type Response = SenderSignature;

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

    use crate::reqwest::PostmarkClient;
    use crate::Query;

    use super::*;

    #[tokio::test]
    async fn get_signature() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/senders/1")).respond_with(
                json_encoded(json!({
                    "Domain": "example.com",
                    "EmailAddress": "john@example.com",
                    "ReplyToEmailAddress": "reply@example.com",
                    "Name": "John",
                    "Confirmed": true,
                    "SPFVerified": false,
                    "DKIMVerified": true,
                    "WeakDKIM": false,
                    "ID": 1
                })),
            ),
        );
        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = GetSignatureRequest::builder().signature_id(1).build();
        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.id, 1);
    }
}
