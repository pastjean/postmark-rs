use std::borrow::Cow;

use crate::Endpoint;
use crate::api::endpoint_with_path_segment;
use crate::api::message_streams::{MessageStream, StreamIdOrName};
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct GetMessageStreamRequest {
    #[serde(skip)]
    pub stream_id: StreamIdOrName,
}

impl Endpoint for GetMessageStreamRequest {
    type Request = GetMessageStreamRequest;
    type Response = MessageStream;

    fn endpoint(&self) -> Cow<'static, str> {
        endpoint_with_path_segment("/message-streams", &self.stream_id.to_string())
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
    use crate::api::message_streams::MessageStreamType;
    use crate::reqwest::PostmarkClient;

    use super::*;

    const STREAM_ID: &str = "broadcasts";

    #[tokio::test]
    pub async fn get_message_stream() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path(
                "GET",
                format!("/message-streams/{STREAM_ID}"),
            ))
            .respond_with(json_encoded(json!({
                "ID": "broadcasts",
                "ServerID": 123456,
                "Name": "Broadcast Stream",
                "Description": "This is my stream to send broadcast messages",
                "MessageStreamType": "Broadcasts",
                "CreatedAt": "2020-07-01T00:00:00-04:00",
                "UpdatedAt": "2020-07-01T00:00:00-04:00",
                "ArchivedAt": null,
                "ExpectedPurgeDate": null,
                "SubscriptionManagementConfiguration": {
                    "UnsubscribeHandlingType": "Postmark"
                }
            }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = GetMessageStreamRequest::builder()
            .stream_id(StreamIdOrName::StreamId(String::from(STREAM_ID)))
            .build();

        let resp = req
            .execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");

        assert_eq!(resp.id, STREAM_ID);
        assert_eq!(resp.message_stream_type, MessageStreamType::Broadcasts);
    }
}
