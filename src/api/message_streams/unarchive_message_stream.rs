use std::borrow::Cow;

use crate::Endpoint;
use crate::api::message_streams::{MessageStream, StreamIdOrName};
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct UnarchiveMessageStreamRequest {
    #[serde(skip)]
    pub stream_id: StreamIdOrName,
}

impl Endpoint for UnarchiveMessageStreamRequest {
    type Request = UnarchiveMessageStreamRequest;
    type Response = MessageStream;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/message-streams/{}/unarchive", self.stream_id).into()
    }

    fn body(&self) -> &Self::Request {
        self
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

    const STREAM_ID: &str = "transactional-dev";

    #[tokio::test]
    pub async fn unarchive_message_stream() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path(
                "POST",
                format!("/message-streams/{STREAM_ID}/unarchive"),
            ))
            .respond_with(json_encoded(json!({
                "ID": STREAM_ID,
                "ServerID": 123457,
                "Name": "Updated Dev Stream",
                "Description": "Updating my dev transactional stream",
                "MessageStreamType": "Transactional",
                "CreatedAt": "2020-07-02T00:00:00-04:00",
                "UpdatedAt": "2020-07-04T00:00:00-04:00",
                "ArchivedAt": null,
                "ExpectedPurgeDate": null,
                "SubscriptionManagementConfiguration": {
                    "UnsubscribeHandlingType": "None"
                }
            }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = UnarchiveMessageStreamRequest::builder()
            .stream_id(StreamIdOrName::StreamId(String::from(STREAM_ID)))
            .build();

        let resp = req.execute(&client).await.expect("json decode");

        assert_eq!(resp.id, STREAM_ID);
        assert_eq!(resp.message_stream_type, MessageStreamType::Transactional);
    }
}
