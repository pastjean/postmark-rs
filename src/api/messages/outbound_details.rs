use std::borrow::Cow;

use serde::Serialize;

use crate::Endpoint;
use crate::api::messages::OutboundMessageDetails;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct OutboundDetailsRequest {
    #[serde(skip)]
    pub message_id: String,
}

impl OutboundDetailsRequest {
    pub fn new(message_id: String) -> Self {
        Self { message_id }
    }
}

pub type OutboundDetailsResponse = OutboundMessageDetails;

impl Endpoint for OutboundDetailsRequest {
    type Request = ();
    type Response = OutboundDetailsResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/messages/outbound/{}/details", self.message_id).into()
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
    use httptest::{Expectation, Server, responders::*};
    use serde_json::json;

    use super::*;
    use crate::Query;
    use crate::reqwest::PostmarkClient;

    #[tokio::test]
    async fn outbound_details_gets_message_details() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path(
                "GET",
                "/messages/outbound/out-msg-1/details",
            ))
            .respond_with(json_encoded(json!({
                "MessageID": "out-msg-1",
                "Subject": "Hello",
                "Status": "Sent",
                "From": "sender@example.com",
                "To": "user@example.com",
                "ReceivedAt": "2023-01-01T00:00:00Z",
                "Tag": "welcome",
                "Recipient": "user@example.com",
                "MessageStream": "outbound"
            }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = OutboundDetailsRequest::new("out-msg-1".to_string());

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode outbound details");

        assert_eq!(resp.message_id, "out-msg-1");
        assert_eq!(resp.status, Some("Sent".to_string()));
        assert_eq!(resp.from, Some("sender@example.com".to_string()));
        assert_eq!(resp.message_stream, Some("outbound".to_string()));
    }
}
