use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::api::domains::Domain;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct RotateDomainDkimRequest {
    #[serde(skip)]
    pub domain_id: isize,
}

pub type RotateDkimResponse = Domain;

impl Endpoint for RotateDomainDkimRequest {
    type Request = ();
    type Response = RotateDkimResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/domains/{}/rotatedkim", self.domain_id).into()
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
    use httptest::{responders::*, Expectation, Server};
    use serde_json::json;

    use super::*;
    use crate::reqwest::PostmarkClient;
    use crate::Query;

    #[tokio::test]
    async fn rotate_dkim_posts_rotate_dkim_path() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("POST", "/domains/11/rotatedkim"))
                .respond_with(json_encoded(json!({
                    "ID": 11,
                    "Name": "example.com"
                }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = RotateDomainDkimRequest::builder().domain_id(11).build();

        assert_eq!(req.method(), http::Method::POST);
        assert_eq!(req.endpoint(), "/domains/11/rotatedkim");

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode rotate dkim");

        assert_eq!(resp.id, 11);
    }
}
