use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::api::message_streams::{
    MessageStream, MessageStreamType, SubscriptionManagementConfiguration,
};
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct CreateMessageStreamRequest {
    #[serde(rename = "ID")]
    #[builder(setter(into))]
    pub id: String,
    #[builder(setter(into))]
    pub name: String,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub message_stream_type: MessageStreamType,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_management_configuration: Option<SubscriptionManagementConfiguration>,
}

pub type CreateMessageStreamResponse = MessageStream;

impl Endpoint for CreateMessageStreamRequest {
    type Request = CreateMessageStreamRequest;
    type Response = CreateMessageStreamResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "/message-streams".into()
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
    async fn create_message_stream_posts_and_decodes_response() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("POST", "/message-streams")).respond_with(
                json_encoded(json!({
                    "ID": "transactional-dev",
                    "ServerID": 12345,
                    "Name": "Dev Stream",
                    "Description": "Dev transactional stream",
                    "MessageStreamType": "Transactional",
                    "CreatedAt": "2020-07-01T00:00:00-04:00",
                    "UpdatedAt": null,
                    "ArchivedAt": null,
                    "ExpectedPurgeDate": null,
                    "SubscriptionManagementConfiguration": {
                        "UnsubscribeHandlingType": "none"
                    }
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = CreateMessageStreamRequest::builder()
            .id("transactional-dev")
            .name("Dev Stream")
            .description("Dev transactional stream")
            .message_stream_type(MessageStreamType::Transactional)
            .build();

        assert_eq!(req.endpoint(), "/message-streams");
        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            json!({
                "ID": "transactional-dev",
                "Name": "Dev Stream",
                "Description": "Dev transactional stream",
                "MessageStreamType": "Transactional",
            })
        );

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode create message stream response");

        assert_eq!(resp.id, "transactional-dev");
    }
}
