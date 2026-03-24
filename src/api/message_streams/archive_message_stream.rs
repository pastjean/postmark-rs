use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::api::message_streams::{ArchiveMessageStreamResponse, StreamIdOrName};
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct ArchiveMessageStreamRequest {
    #[serde(skip)]
    pub stream_id: StreamIdOrName,
}

impl Endpoint for ArchiveMessageStreamRequest {
    type Request = ArchiveMessageStreamRequest;
    type Response = ArchiveMessageStreamResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/message-streams/{}/archive", self.stream_id).into()
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
    async fn archive_message_stream_posts_and_decodes_response() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path(
                "POST",
                "/message-streams/transactional-dev/archive",
            ))
            .respond_with(json_encoded(json!({
                "ID": "transactional-dev",
                "ServerID": 12345,
                "ExpectedPurgeDate": "2020-08-30T12:30:00-04:00"
            }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = ArchiveMessageStreamRequest::builder()
            .stream_id(StreamIdOrName::StreamId("transactional-dev".to_string()))
            .build();

        assert_eq!(req.method(), http::Method::POST);
        assert_eq!(req.endpoint(), "/message-streams/transactional-dev/archive");

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode archive message stream response");

        assert_eq!(resp.id, "transactional-dev");
        assert_eq!(resp.server_id, 12345);
    }
}
