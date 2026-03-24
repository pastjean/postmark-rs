use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::api::message_streams::{MessageStream, StreamIdOrName};
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct GetMessageStreamRequest {
    #[serde(skip)]
    pub stream_id: StreamIdOrName,
}

pub type GetMessageStreamResponse = MessageStream;

impl Endpoint for GetMessageStreamRequest {
    type Request = GetMessageStreamRequest;
    type Response = GetMessageStreamResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/message-streams/{}", self.stream_id).into()
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
    use httptest::{responders::*, Expectation, Server};
    use serde_json::json;

    use crate::reqwest::PostmarkClient;
    use crate::Query;

    use super::*;

    #[tokio::test]
    async fn get_message_stream_gets_stream_and_decodes_response() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/message-streams/outbound"))
                .respond_with(json_encoded(json!({
                    "ID": "outbound",
                    "ServerID": 12345,
                    "Name": "Transactional",
                    "Description": "Default transactional stream",
                    "MessageStreamType": "Transactional",
                    "CreatedAt": "2020-07-01T00:00:00-04:00",
                    "UpdatedAt": null,
                    "ArchivedAt": null,
                    "ExpectedPurgeDate": null,
                    "SubscriptionManagementConfiguration": {
                        "UnsubscribeHandlingType": "none"
                    }
                }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = GetMessageStreamRequest::builder()
            .stream_id(StreamIdOrName::StreamId("outbound".to_string()))
            .build();

        assert_eq!(req.method(), http::Method::GET);
        assert_eq!(req.endpoint(), "/message-streams/outbound");

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode get message stream response");

        assert_eq!(resp.id, "outbound");
        assert_eq!(resp.server_id, 12345);
    }
}
