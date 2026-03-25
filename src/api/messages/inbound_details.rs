use std::borrow::Cow;

use serde::Serialize;

use crate::Endpoint;
use crate::api::messages::InboundMessageDetails;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct InboundDetailsRequest {
    #[serde(skip)]
    pub message_id: String,
}

impl InboundDetailsRequest {
    pub fn new(message_id: String) -> Self {
        Self { message_id }
    }
}

pub type InboundDetailsResponse = InboundMessageDetails;

impl Endpoint for InboundDetailsRequest {
    type Request = ();
    type Response = InboundDetailsResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/messages/inbound/{}/details", self.message_id).into()
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
    async fn inbound_details_gets_message_details() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path(
                "GET",
                "/messages/inbound/in-msg-1/details",
            ))
            .respond_with(json_encoded(json!({
                "MessageID": "in-msg-1",
                "Subject": "Reply",
                "Status": "Received",
                "From": "sender@example.com",
                "To": "inbound@example.com",
                "MailboxHash": "box-1",
                "ReceivedAt": "2023-01-01T00:00:00Z",
                "Tag": null,
                "Recipient": "inbound@example.com",
                "MessageStream": "inbound"
            }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = InboundDetailsRequest::new("in-msg-1".to_string());

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode inbound details");

        assert_eq!(resp.message_id, "in-msg-1");
        assert_eq!(resp.mailbox_hash, Some("box-1".to_string()));
        assert_eq!(resp.message_stream, Some("inbound".to_string()));
    }
}
