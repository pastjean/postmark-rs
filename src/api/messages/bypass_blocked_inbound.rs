use std::borrow::Cow;

use serde::Serialize;

use crate::Endpoint;
use crate::api::messages::MessageActionResponse;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BypassBlockedInboundRequest {
    #[serde(skip)]
    pub message_id: String,
}

impl BypassBlockedInboundRequest {
    pub fn new(message_id: String) -> Self {
        Self { message_id }
    }
}

pub type BypassBlockedInboundResponse = MessageActionResponse;

impl Endpoint for BypassBlockedInboundRequest {
    type Request = ();
    type Response = BypassBlockedInboundResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/messages/inbound/{}/bypass", self.message_id).into()
    }

    fn body(&self) -> &Self::Request {
        &()
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

    use super::*;
    use crate::Query;
    use crate::reqwest::PostmarkClient;

    #[tokio::test]
    async fn bypass_blocked_inbound_puts_bypass() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path(
                "PUT",
                "/messages/inbound/in-msg-1/bypass",
            ))
            .respond_with(json_encoded(json!({ "Message": "OK", "ErrorCode": 0 }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = BypassBlockedInboundRequest::new("in-msg-1".to_string());

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode bypass response");

        assert_eq!(resp.message, "OK");
        assert_eq!(resp.error_code, Some(0));
    }
}
