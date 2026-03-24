use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::api::domains::Domain;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct GetDomainRequest {
    #[serde(skip)]
    pub domain_id: isize,
}

pub type GetDomainResponse = Domain;

impl Endpoint for GetDomainRequest {
    type Request = GetDomainRequest;
    type Response = GetDomainResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/domains/{}", self.domain_id).into()
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

    use super::*;
    use crate::reqwest::PostmarkClient;
    use crate::Query;

    #[tokio::test]
    async fn get_domain_gets_domain_by_id() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/domains/11")).respond_with(
                json_encoded(json!({
                    "ID": 11,
                    "Name": "example.com"
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = GetDomainRequest::builder().domain_id(11).build();

        assert_eq!(req.method(), http::Method::GET);
        assert_eq!(req.endpoint(), "/domains/11");

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode get domain");

        assert_eq!(resp.id, 11);
        assert_eq!(resp.name, "example.com");
    }
}
