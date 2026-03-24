use std::borrow::Cow;

use crate::Endpoint;
use crate::api::bounce::{BounceId, BounceInfo};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct ActivateBounceRequest {
    #[builder(setter(into))]
    #[serde(skip)]
    pub bounce_id: BounceId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActivateBounceResponse {
    pub message: String,
    pub bounce: BounceInfo,
}

impl Endpoint for ActivateBounceRequest {
    type Request = ActivateBounceRequest;
    type Response = ActivateBounceResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/bounces/{}/activate", self.bounce_id).into()
    }

    fn body(&self) -> &Self::Request {
        self
    }

    fn method(&self) -> http::Method {
        http::Method::PUT
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
    pub async fn activate_bounce() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("PUT", "/bounces/42/activate"))
                .respond_with(json_encoded(json!({
                    "Message": "Bounce activated",
                    "Bounce": {
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
                        "Inactive": false,
                        "CanActivate": false,
                        "Content": null,
                        "Subject": null
                    }
                }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = ActivateBounceRequest::builder().bounce_id(42).build();
        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.bounce.id, 42);
    }
}
