use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::api::signatures::SignatureActionResponse;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteSignatureRequest {
    #[serde(skip)]
    pub signature_id: isize,
}

pub type DeleteSignatureResponse = SignatureActionResponse;

impl Endpoint for DeleteSignatureRequest {
    type Request = DeleteSignatureRequest;
    type Response = DeleteSignatureResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/senders/{}", self.signature_id).into()
    }

    fn body(&self) -> &Self::Request {
        self
    }

    fn method(&self) -> http::Method {
        http::Method::DELETE
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
    async fn delete_signature_deletes_sender() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("DELETE", "/senders/22")).respond_with(
                json_encoded(json!({
                    "ErrorCode": 0,
                    "Message": "OK"
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = DeleteSignatureRequest::builder().signature_id(22).build();

        assert_eq!(req.method(), http::Method::DELETE);
        assert_eq!(req.endpoint(), "/senders/22");

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode delete signature");

        assert_eq!(resp.error_code, 0);
    }
}
