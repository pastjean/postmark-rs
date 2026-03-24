use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::api::domains::Domain;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct EditDomainRequest {
    #[serde(skip)]
    pub domain_id: isize,
    #[builder(setter(into))]
    pub name: String,
}

pub type EditDomainResponse = Domain;

impl Endpoint for EditDomainRequest {
    type Request = EditDomainRequest;
    type Response = EditDomainResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/domains/{}", self.domain_id).into()
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
    async fn edit_domain_puts_domain() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("PUT", "/domains/11")).respond_with(
                json_encoded(json!({
                    "ID": 11,
                    "Name": "edited.example.com"
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = EditDomainRequest::builder()
            .domain_id(11)
            .name("edited.example.com")
            .build();

        assert_eq!(req.method(), http::Method::PUT);
        assert_eq!(req.endpoint(), "/domains/11");
        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            json!({ "Name": "edited.example.com" })
        );

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode edit domain");

        assert_eq!(resp.name, "edited.example.com");
    }
}
