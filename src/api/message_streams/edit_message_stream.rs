use std::borrow::Cow;

use crate::api::message_streams::{
    MessageStream, StreamIdOrName, SubscriptionManagementConfiguration,
};
use crate::Endpoint;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct EditMessageStreamRequest {
    #[serde(skip)]
    pub stream_id: StreamIdOrName,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_management_configuration: Option<SubscriptionManagementConfiguration>,
}

impl Endpoint for EditMessageStreamRequest {
    type Request = EditMessageStreamRequest;
    type Response = MessageStream;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/message-streams/{}", self.stream_id).into()
    }

    fn body(&self) -> &Self::Request {
        self
    }

    fn method(&self) -> http::Method {
        http::Method::PATCH
    }
}

#[cfg(test)]
mod tests {
    use httptest::matchers::request;
    use httptest::{responders::*, Expectation, Server};
    use serde_json::json;

    use crate::api::message_streams::MessageStreamType;
    use crate::reqwest::PostmarkClient;
    use crate::Query;

    use super::*;

    const STREAM_ID: &str = "transactional-dev";

    #[tokio::test]
    pub async fn edit_message_stream() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path(
                "PATCH",
                format!("/message-streams/{STREAM_ID}"),
            ))
            .respond_with(json_encoded(json!({
                "ID": "transactional-dev",
                "ServerID": 123457,
                "Name": "Updated Dev Stream",
                "Description": "Updating my dev transactional stream",
                "MessageStreamType": "Transactional",
                "CreatedAt": "2020-07-02T00:00:00-04:00",
                "UpdatedAt": "2020-07-03T00:00:00-04:00",
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

        let req = EditMessageStreamRequest::builder()
            .stream_id(StreamIdOrName::StreamId(String::from(STREAM_ID)))
            .name("Updated Dev Stream")
            .description("Updating my dev transactional stream")
            .build();

        let resp = req.execute(&client).await.expect("json decode");

        assert_eq!(resp.id, STREAM_ID);
        assert_eq!(resp.message_stream_type, MessageStreamType::Transactional);
    }
}
