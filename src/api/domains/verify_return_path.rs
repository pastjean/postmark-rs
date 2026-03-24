use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::api::domains::Domain;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct VerifyDomainReturnPathRequest {
    #[serde(skip)]
    pub domain_id: isize,
}

pub type VerifyReturnPathResponse = Domain;

impl Endpoint for VerifyDomainReturnPathRequest {
    type Request = ();
    type Response = VerifyReturnPathResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/domains/{}/verifyReturnPath", self.domain_id).into()
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
    async fn verify_return_path_puts_verify_return_path() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("PUT", "/domains/11/verifyReturnPath"))
                .respond_with(json_encoded(json!({
                    "ID": 11,
                    "Name": "example.com"
                }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = VerifyDomainReturnPathRequest::builder()
            .domain_id(11)
            .build();

        assert_eq!(req.method(), http::Method::PUT);
        assert_eq!(req.endpoint(), "/domains/11/verifyReturnPath");

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode verify return path");

        assert_eq!(resp.id, 11);
    }
}
