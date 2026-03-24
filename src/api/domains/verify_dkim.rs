use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::api::domains::Domain;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct VerifyDomainDkimRequest {
    #[serde(skip)]
    pub domain_id: isize,
}

pub type VerifyDkimResponse = Domain;

impl Endpoint for VerifyDomainDkimRequest {
    type Request = ();
    type Response = VerifyDkimResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/domains/{}/verifyDkim", self.domain_id).into()
    }

    fn body(&self) -> &Self::Request {
        &()
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
    async fn verify_dkim_puts_verify_dkim_path() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("PUT", "/domains/11/verifyDkim"))
                .respond_with(json_encoded(json!({
                    "ID": 11,
                    "Name": "example.com"
                }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = VerifyDomainDkimRequest::builder().domain_id(11).build();

        assert_eq!(req.method(), http::Method::PUT);
        assert_eq!(req.endpoint(), "/domains/11/verifyDkim");

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode verify dkim");

        assert_eq!(resp.id, 11);
    }
}
