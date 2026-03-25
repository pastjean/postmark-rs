use std::borrow::Cow;

use crate::Endpoint;
use crate::api::server::Server;
use crate::api::{DEFAULT_PAGE_COUNT, DEFAULT_PAGE_OFFSET, endpoint_with_query};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use url::form_urlencoded::Serializer;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct ListServersRequest {
    #[serde(skip)]
    #[builder(default = DEFAULT_PAGE_COUNT)]
    pub count: i64,
    #[serde(skip)]
    #[builder(default = DEFAULT_PAGE_OFFSET)]
    pub offset: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListServersResponse {
    pub total_count: i64,
    pub servers: Vec<Server>,
}

impl Endpoint for ListServersRequest {
    type Request = ListServersRequest;
    type Response = ListServersResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        let mut serializer = Serializer::new(String::new());
        serializer.append_pair("count", &self.count.to_string());
        serializer.append_pair("offset", &self.offset.to_string());
        endpoint_with_query("/servers", serializer.finish())
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
    use httptest::{Expectation, Server as HttpServer, responders::*};
    use serde_json::json;

    use crate::Query;
    use crate::reqwest::PostmarkClient;

    use super::*;

    #[tokio::test]
    pub async fn list_servers() {
        let server = HttpServer::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/servers")).respond_with(
                json_encoded(json!({
                    "TotalCount": 1,
                    "Servers": [{
                        "ID": 1,
                        "Name": "Staging Testing",
                        "ApiTokens": ["server token"],
                        "SmtpApiActivated": true
                    }]
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = ListServersRequest::builder().count(100).offset(0).build();

        let resp = req.execute(&client).await.expect("json decode");

        assert_eq!(resp.total_count, 1);
        assert_eq!(resp.servers[0].id, 1);
    }

    #[test]
    fn list_servers_endpoint_encodes_query_consistently() {
        let req = ListServersRequest::builder().count(100).offset(0).build();
        assert_eq!(req.endpoint(), "/servers?count=100&offset=0");
    }

    #[test]
    fn list_servers_uses_default_pagination() {
        let req = ListServersRequest::builder().build();
        assert_eq!(req.endpoint(), "/servers?count=100&offset=0");
    }
}
