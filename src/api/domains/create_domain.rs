use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::api::domains::Domain;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct CreateDomainRequest {
    #[builder(setter(into))]
    pub name: String,
}

pub type CreateDomainResponse = Domain;

impl Endpoint for CreateDomainRequest {
    type Request = CreateDomainRequest;
    type Response = CreateDomainResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "/domains".into()
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

    use super::*;
    use crate::reqwest::PostmarkClient;
    use crate::Query;

    #[tokio::test]
    async fn create_domain_posts_domain() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("POST", "/domains")).respond_with(
                json_encoded(json!({
                    "ID": 11,
                    "Name": "example.com"
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = CreateDomainRequest::builder().name("example.com").build();

        assert_eq!(req.method(), http::Method::POST);
        assert_eq!(req.endpoint(), "/domains");
        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            json!({ "Name": "example.com" })
        );

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode create domain");

        assert_eq!(resp.id, 11);
    }
}
