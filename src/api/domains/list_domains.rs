use std::borrow::Cow;

use crate::Endpoint;
use crate::api::domains::DomainSummary;
use crate::api::{DEFAULT_PAGE_COUNT, DEFAULT_PAGE_OFFSET, endpoint_with_query};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use url::form_urlencoded::Serializer;

/// List domains with pagination.
///
/// ```
/// use postmark::api::domains::ListDomainsRequest;
/// let req = ListDomainsRequest::builder()
///   .count(50)
///   .offset(0)
///   .build();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct ListDomainsRequest {
    /// Number of records to return per request. Max 500.
    #[serde(skip)]
    #[builder(default = DEFAULT_PAGE_COUNT)]
    pub count: i64,
    /// Number of records to skip.
    #[serde(skip)]
    #[builder(default = DEFAULT_PAGE_OFFSET)]
    pub offset: i64,
}

/// Response for the [`ListDomainsRequest`] endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListDomainsResponse {
    /// Total number of domains matching the query. May exceed the number returned in a single call.
    pub total_count: i64,
    /// List of domain summaries.
    pub domains: Vec<DomainSummary>,
}

impl Endpoint for ListDomainsRequest {
    type Request = ListDomainsRequest;
    type Response = ListDomainsResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        let mut serializer = Serializer::new(String::new());
        serializer.append_pair("count", &self.count.to_string());
        serializer.append_pair("offset", &self.offset.to_string());
        endpoint_with_query("/domains", serializer.finish())
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
    use httptest::{Expectation, Server, responders::*};
    use serde_json::json;

    use crate::Query;
    use crate::reqwest::PostmarkClient;

    use super::*;

    #[tokio::test]
    pub async fn list_domains() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/domains")).respond_with(
                json_encoded(json!({
                    "TotalCount": 2,
                    "Domains": [
                        {
                            "Name": "postmarkapp.com",
                            "SPFVerified": true,
                            "DKIMVerified": true,
                            "WeakDKIM": false,
                            "ReturnPathDomainVerified": false,
                            "ID": 36735
                        },
                        {
                            "Name": "example.com",
                            "SPFVerified": true,
                            "DKIMVerified": true,
                            "WeakDKIM": false,
                            "ReturnPathDomainVerified": true,
                            "ID": 81605
                        }
                    ]
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = ListDomainsRequest::builder().count(50).offset(0).build();

        let resp = req
            .execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");

        assert_eq!(resp.total_count, 2);
        assert_eq!(resp.domains.len(), 2);
        assert_eq!(resp.domains[0].name, "postmarkapp.com");
        assert_eq!(resp.domains[0].id, 36735);
    }

    #[test]
    fn list_domains_uses_default_pagination() {
        let req = ListDomainsRequest::builder().build();
        assert_eq!(req.endpoint(), "/domains?count=100&offset=0");
    }
}
