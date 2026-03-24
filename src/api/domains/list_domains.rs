use std::borrow::Cow;

use serde::Serialize;

use crate::api::domains::ListDomainsResponse;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct ListDomainsRequest;

impl Endpoint for ListDomainsRequest {
    type Request = ListDomainsRequest;
    type Response = ListDomainsResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "/domains".into()
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
    async fn list_domains_gets_domains() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/domains")).respond_with(
                json_encoded(json!({
                    "TotalCount": 1,
                    "Domains": [
                        {
                            "ID": 11,
                            "Name": "example.com"
                        }
                    ]
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = ListDomainsRequest {};

        assert_eq!(req.method(), http::Method::GET);
        assert_eq!(req.endpoint(), "/domains");

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode list domains");

        assert_eq!(resp.total_count, 1);
        assert_eq!(resp.domains[0].id, 11);
    }
}
