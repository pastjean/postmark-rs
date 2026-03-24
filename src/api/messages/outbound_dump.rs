use std::borrow::Cow;

use serde::Serialize;

use crate::api::messages::MessageDump;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct OutboundDumpRequest {
    #[serde(skip)]
    pub message_id: String,
}

impl OutboundDumpRequest {
    pub fn new(message_id: String) -> Self {
        Self { message_id }
    }
}

pub type OutboundDumpResponse = MessageDump;

impl Endpoint for OutboundDumpRequest {
    type Request = ();
    type Response = OutboundDumpResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/messages/outbound/{}/dump", self.message_id).into()
    }

    fn body(&self) -> &Self::Request {
        &()
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
    async fn outbound_dump_gets_raw_body() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path(
                "GET",
                "/messages/outbound/out-msg-1/dump",
            ))
            .respond_with(json_encoded(json!({ "Body": "raw source" }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = OutboundDumpRequest::new("out-msg-1".to_string());

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode outbound dump");

        assert_eq!(resp.body, "raw source");
    }
}
