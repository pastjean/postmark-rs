use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use url::form_urlencoded::Serializer;

use crate::api::server::GetServerResponse;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct ListServersRequest {
    pub count: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListServersResponse {
    pub total_count: isize,
    pub servers: Vec<GetServerResponse>,
}

impl Endpoint for ListServersRequest {
    type Request = ListServersRequest;
    type Response = ListServersResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        let mut serializer = Serializer::new(String::new());

        if let Some(count) = self.count {
            serializer.append_pair("count", &count.to_string());
        }

        if let Some(offset) = self.offset {
            serializer.append_pair("offset", &offset.to_string());
        }

        let query = serializer.finish();

        if query.is_empty() {
            "/servers".into()
        } else {
            format!("/servers?{query}").into()
        }
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

    use crate::reqwest::PostmarkClient;
    use crate::Query;

    use super::*;

    #[tokio::test]
    async fn list_servers_gets_servers_and_decodes_response() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/servers")).respond_with(
                json_encoded(json!({
                    "TotalCount": 2,
                    "Servers": [
                        {
                            "ID": 12345,
                            "Name": "Primary",
                            "ApiTokens": ["token-1"]
                        },
                        {
                            "ID": 67890,
                            "Name": "Staging",
                            "ApiTokens": ["token-2"]
                        }
                    ]
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = ListServersRequest::default();

        assert_eq!(req.method(), http::Method::GET);
        assert_eq!(req.endpoint(), "/servers");

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode list servers response");

        assert_eq!(resp.total_count, 2);
        assert_eq!(resp.servers[0].id, 12345);
        assert_eq!(resp.servers[1].name, "Staging");
    }

    #[test]
    fn list_servers_with_count_and_offset_sets_query_path() {
        let req = ListServersRequest {
            count: Some(10),
            offset: Some(20),
        };

        let endpoint = req.endpoint();
        let endpoint = endpoint.as_ref();

        assert!(endpoint.starts_with("/servers?"));
        assert!(endpoint.contains("count=10"));
        assert!(endpoint.contains("offset=20"));
    }
}
