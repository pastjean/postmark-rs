use std::borrow::Cow;

use crate::Endpoint;
use crate::api::bounce::{BounceId, BounceInfo};
use crate::api::endpoint_with_path_segment;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct GetBounceRequest {
    #[builder(setter(into))]
    #[serde(skip)]
    pub bounce_id: BounceId,
}

impl Endpoint for GetBounceRequest {
    type Request = GetBounceRequest;
    type Response = BounceInfo;

    fn endpoint(&self) -> Cow<'static, str> {
        endpoint_with_path_segment("/bounces", &self.bounce_id.to_string())
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
    pub async fn get_bounce() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/bounces/42")).respond_with(
                json_encoded(json!({
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
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = GetBounceRequest::builder().bounce_id(42).build();
        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.id, 42);
    }
}
