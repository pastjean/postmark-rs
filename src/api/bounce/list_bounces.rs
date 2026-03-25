use std::borrow::Cow;

use crate::Endpoint;
use crate::api::bounce::BounceInfo;
use crate::api::{
    DEFAULT_PAGE_COUNT, DEFAULT_PAGE_OFFSET, endpoint_with_query,
};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use url::form_urlencoded::Serializer;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct ListBouncesRequest {
    #[serde(skip)]
    #[builder(default = DEFAULT_PAGE_COUNT)]
    pub count: i64,
    #[serde(skip)]
    #[builder(default = DEFAULT_PAGE_OFFSET)]
    pub offset: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListBouncesResponse {
    pub total_count: i64,
    pub bounces: Vec<BounceInfo>,
}

impl Endpoint for ListBouncesRequest {
    type Request = ListBouncesRequest;
    type Response = ListBouncesResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        let mut serializer = Serializer::new(String::new());
        serializer.append_pair("count", &self.count.to_string());
        serializer.append_pair("offset", &self.offset.to_string());
        endpoint_with_query("/bounces", serializer.finish())
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
    pub async fn list_bounces() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/bounces")).respond_with(
                json_encoded(json!({
                    "TotalCount": 1,
                    "Bounces": [{
                        "ID": 42,
                        "Type": "HardBounce",
                        "TypeCode": 1,
                        "Name": "Hard bounce",
                        "Tag": "Invitation",
                        "MessageID": "0aa96361",
                        "Description": "The server was unable to deliver your message",
                        "Details": "relay=none, delay=0.16",
                        "Email": "zaphod@example.com",
                        "BouncedAt": "2019-06-18T07:27:19.0000000-04:00",
                        "DumpAvailable": true,
                        "Inactive": true,
                        "CanActivate": true,
                        "Content": null,
                        "Subject": null
                    }]
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = ListBouncesRequest::builder().count(50).offset(0).build();
        let resp = req.execute(&client).await.expect("json decode");

        assert_eq!(resp.total_count, 1);
        assert_eq!(resp.bounces[0].id, 42);
    }

    #[test]
    fn list_bounces_uses_default_pagination() {
        let req = ListBouncesRequest::builder().build();
        assert_eq!(req.endpoint(), "/bounces?count=100&offset=0");
    }
}
