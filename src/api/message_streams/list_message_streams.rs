use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use url::form_urlencoded::Serializer;

use crate::api::message_streams::{MessageStream, MessageStreamTypeFilter};
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, Default, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct ListMessageStreamsRequest {
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_stream_type: Option<MessageStreamTypeFilter>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_archived_streams: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListMessageStreamsResponse {
    pub message_streams: Vec<MessageStream>,
    pub total_count: isize,
}

impl Endpoint for ListMessageStreamsRequest {
    type Request = ListMessageStreamsRequest;
    type Response = ListMessageStreamsResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        let mut serializer = Serializer::new(String::new());

        if let Some(ref message_stream_type) = self.message_stream_type {
            serializer.append_pair(
                "MessageStreamType",
                match message_stream_type {
                    MessageStreamTypeFilter::All => "All",
                    MessageStreamTypeFilter::Inbound => "Inbound",
                    MessageStreamTypeFilter::Broadcasts => "Broadcasts",
                    MessageStreamTypeFilter::Transactional => "Transactional",
                },
            );
        }

        if let Some(include_archived_streams) = self.include_archived_streams {
            serializer.append_pair(
                "IncludeArchivedStreams",
                &include_archived_streams.to_string(),
            );
        }

        let query = serializer.finish();

        if query.is_empty() {
            "/message-streams".into()
        } else {
            format!("/message-streams?{query}").into()
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
    use httptest::{responders::*, Expectation, Server};
    use serde_json::json;

    use crate::reqwest::PostmarkClient;
    use crate::Query;

    use super::*;

    #[tokio::test]
    async fn list_message_streams_gets_streams_and_decodes_response() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/message-streams")).respond_with(
                json_encoded(json!({
                    "TotalCount": 1,
                    "MessageStreams": [
                        {
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
                        }
                    ]
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = ListMessageStreamsRequest::default();

        assert_eq!(req.method(), http::Method::GET);
        assert_eq!(req.endpoint(), "/message-streams");

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode list message streams response");

        assert_eq!(resp.total_count, 1);
        assert_eq!(resp.message_streams[0].id, "outbound");
    }

    #[test]
    fn list_message_streams_query_params_are_encoded() {
        let req = ListMessageStreamsRequest::builder()
            .message_stream_type(MessageStreamTypeFilter::All)
            .include_archived_streams(true)
            .build();

        let endpoint = req.endpoint();
        let endpoint = endpoint.as_ref();

        assert!(endpoint.starts_with("/message-streams?"));
        assert!(endpoint.contains("MessageStreamType=All"));
        assert!(endpoint.contains("IncludeArchivedStreams=true"));
    }
}
