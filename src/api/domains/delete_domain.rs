use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::api::domains::DomainActionResponse;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteDomainRequest {
    #[serde(skip)]
    pub domain_id: isize,
}

pub type DeleteDomainResponse = DomainActionResponse;

impl Endpoint for DeleteDomainRequest {
    type Request = DeleteDomainRequest;
    type Response = DeleteDomainResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/domains/{}", self.domain_id).into()
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
    async fn delete_domain_deletes_by_id() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("DELETE", "/domains/11")).respond_with(
                json_encoded(json!({
                    "ErrorCode": 0,
                    "Message": "OK"
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = DeleteDomainRequest::builder().domain_id(11).build();

        assert_eq!(req.method(), http::Method::DELETE);
        assert_eq!(req.endpoint(), "/domains/11");

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode delete domain");

        assert_eq!(resp.error_code, 0);
    }
}
