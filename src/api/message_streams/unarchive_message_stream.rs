use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::api::message_streams::{MessageStream, StreamIdOrName};
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct UnarchiveMessageStreamRequest {
    #[serde(skip)]
    pub stream_id: StreamIdOrName,
}

pub type UnarchiveMessageStreamResponse = MessageStream;

impl Endpoint for UnarchiveMessageStreamRequest {
    type Request = UnarchiveMessageStreamRequest;
    type Response = UnarchiveMessageStreamResponse;

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
    use httptest::{responders::*, Expectation, Server};
    use serde_json::json;

    use crate::reqwest::PostmarkClient;
    use crate::Query;

    use super::*;

    #[tokio::test]
    async fn unarchive_message_stream_posts_and_decodes_response() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path(
                "POST",
                "/message-streams/transactional-dev/unarchive",
            ))
            .respond_with(json_encoded(json!({
                "ID": "transactional-dev",
                "ServerID": 12345,
                "Name": "Dev Stream",
                "Description": "Dev transactional stream",
                "MessageStreamType": "Transactional",
                "CreatedAt": "2020-07-01T00:00:00-04:00",
                "UpdatedAt": "2020-07-02T00:00:00-04:00",
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

        let req = UnarchiveMessageStreamRequest::builder()
            .stream_id(StreamIdOrName::StreamId("transactional-dev".to_string()))
            .build();

        assert_eq!(req.method(), http::Method::POST);
        assert_eq!(
            req.endpoint(),
            "/message-streams/transactional-dev/unarchive"
        );

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode unarchive message stream response");

        assert_eq!(resp.id, "transactional-dev");
        assert_eq!(resp.archived_at, None);
    }
}
