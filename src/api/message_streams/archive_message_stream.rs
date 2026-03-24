use std::borrow::Cow;

use crate::api::message_streams::StreamIdOrName;
use crate::Endpoint;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct ArchiveMessageStreamRequest {
    #[serde(skip)]
    pub stream_id: StreamIdOrName,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ArchiveMessageStreamResponse {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "ServerID")]
    pub server_id: isize,
    pub expected_purge_date: String,
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

    const STREAM_ID: &str = "transactional-dev";

    #[tokio::test]
    pub async fn archive_message_stream() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path(
                "POST",
                format!("/message-streams/{STREAM_ID}/archive"),
            ))
            .respond_with(json_encoded(json!({
                "ID": STREAM_ID,
                "ServerID": 123457,
                "ExpectedPurgeDate": "2020-08-30T12:30:00.00-04:00"
            }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = ArchiveMessageStreamRequest::builder()
            .stream_id(StreamIdOrName::StreamId(String::from(STREAM_ID)))
            .build();

        let resp = req.execute(&client).await.expect("json decode");

        assert_eq!(resp.id, STREAM_ID);
    }
}
