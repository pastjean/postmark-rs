use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::Endpoint;
use crate::api::signatures::{SenderSignature, SignatureId};

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct RequestNewSignatureDkimRequest {
    #[builder(setter(into))]
    #[serde(skip)]
    pub signature_id: SignatureId,
}

pub type RequestNewDkimResponse = SenderSignature;

impl Endpoint for RequestNewSignatureDkimRequest {
    type Request = ();
    type Response = RequestNewDkimResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/senders/{}/requestnewdkim", self.signature_id).into()
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
    async fn request_new_dkim_posts_request_new_dkim_path() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("POST", "/senders/22/requestnewdkim"))
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

        let req = RequestNewSignatureDkimRequest::builder()
            .signature_id(22)
            .build();

        assert_eq!(req.method(), http::Method::POST);
        assert_eq!(req.endpoint(), "/senders/22/requestnewdkim");

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode request new dkim");

        assert_eq!(resp.id, 22);
    }
}
