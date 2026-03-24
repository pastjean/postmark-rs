use std::borrow::Cow;

use serde::Serialize;

use crate::Endpoint;
use crate::api::messages::MessageActionResponse;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RetryFailedInboundRequest {
    #[serde(skip)]
    pub message_id: String,
}

impl RetryFailedInboundRequest {
    pub fn new(message_id: String) -> Self {
        Self { message_id }
    }
}

pub type RetryFailedInboundResponse = MessageActionResponse;

impl Endpoint for RetryFailedInboundRequest {
    type Request = ();
    type Response = RetryFailedInboundResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/messages/inbound/{}/retry", self.message_id).into()
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
    async fn retry_failed_inbound_puts_retry() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path(
                "PUT",
                "/messages/inbound/in-msg-1/retry",
            ))
            .respond_with(json_encoded(json!({ "Message": "OK", "ErrorCode": 0 }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = RetryFailedInboundRequest::new("in-msg-1".to_string());

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode retry response");

        assert_eq!(resp.message, "OK");
        assert_eq!(resp.error_code, Some(0));
    }
}
