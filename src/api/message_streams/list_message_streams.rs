use std::borrow::Cow;

use crate::Endpoint;
use crate::api::message_streams::{MessageStream, MessageStreamType};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct ListMessageStreamsRequest {
    #[serde(skip)]
    #[builder(default, setter(into, strip_option))]
    pub message_stream_type: Option<MessageStreamType>,
    #[serde(skip)]
    #[builder(default, setter(into, strip_option))]
    pub include_archived_streams: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListMessageStreamsResponse {
    pub message_streams: Vec<MessageStream>,
    pub total_count: i64,
}

impl Endpoint for ListMessageStreamsRequest {
    type Request = ListMessageStreamsRequest;
    type Response = ListMessageStreamsResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        let mut query = Vec::new();
        if let Some(message_stream_type) = &self.message_stream_type {
            query.push(format!("MessageStreamType={message_stream_type}"));
        }
        if let Some(include_archived_streams) = self.include_archived_streams {
            query.push(format!("IncludeArchivedStreams={include_archived_streams}"));
        }

        if query.is_empty() {
            "/message-streams".into()
        } else {
            format!("/message-streams?{}", query.join("&")).into()
        }
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
    use crate::reqwest::PostmarkClient;

    use super::*;

    #[tokio::test]
    pub async fn list_message_streams() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/message-streams")).respond_with(
                json_encoded(json!({
                    "MessageStreams": [{
                        "ID": "outbound",
                        "ServerID": 123457,
                        "Name": "Transactional Stream",
                        "Description": "This is my stream to send transactional messages",
                        "MessageStreamType": "Transactional",
                        "CreatedAt": "2020-07-01T00:00:00-04:00",
                        "UpdatedAt": "2020-07-05T00:00:00-04:00",
                        "ArchivedAt": null,
                        "ExpectedPurgeDate": null,
                        "SubscriptionManagementConfiguration": {
                            "UnsubscribeHandlingType": "None"
                        }
                    }],
                    "TotalCount": 1
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = ListMessageStreamsRequest::builder()
            .message_stream_type(MessageStreamType::All)
            .include_archived_streams(true)
            .build();

        assert_eq!(
            req.endpoint(),
            "/message-streams?MessageStreamType=All&IncludeArchivedStreams=true"
        );

        let resp = req
            .execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");

        assert_eq!(resp.total_count, 1);
        assert_eq!(resp.message_streams[0].id, "outbound");
    }
}
