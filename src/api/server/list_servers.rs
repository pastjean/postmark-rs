use std::borrow::Cow;

use crate::Endpoint;
use crate::api::server::Server;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct ListServersRequest {
    #[serde(skip)]
    pub count: isize,
    #[serde(skip)]
    pub offset: isize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListServersResponse {
    pub total_count: isize,
    pub servers: Vec<Server>,
}

impl Endpoint for ListServersRequest {
    type Request = ListServersRequest;
    type Response = ListServersResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/servers?count={}&offset={}", self.count, self.offset).into()
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
}
