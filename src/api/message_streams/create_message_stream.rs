use std::borrow::Cow;

use crate::Endpoint;
use crate::api::message_streams::{
    MessageStream, MessageStreamType, SubscriptionManagementConfiguration,
};
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct CreateMessageStreamRequest {
    pub id: String,
    pub name: String,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub message_stream_type: MessageStreamType,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_management_configuration: Option<SubscriptionManagementConfiguration>,
}

impl Endpoint for CreateMessageStreamRequest {
    type Request = CreateMessageStreamRequest;
    type Response = MessageStream;

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
    use httptest::{Expectation, Server, responders::*};
    use serde_json::json;

    use crate::Query;
    use crate::api::message_streams::UnsubscribeHandlingType;
    use crate::reqwest::PostmarkClient;

    use super::*;

    #[tokio::test]
    pub async fn create_message_stream() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("POST", "/message-streams")).respond_with(
                json_encoded(json!({
                    "ID": "transactional-dev",
                    "ServerID": 123457,
                    "Name": "My Dev Transactional Stream",
                    "Description": "This is my second transactional stream",
                    "MessageStreamType": "Transactional",
                    "CreatedAt": "2020-07-02T00:00:00-04:00",
                    "UpdatedAt": "2020-07-02T00:00:00-04:00",
                    "ArchivedAt": null,
                    "ExpectedPurgeDate": null,
                    "SubscriptionManagementConfiguration": {
                        "UnsubscribeHandlingType": "None"
                    }
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = CreateMessageStreamRequest::builder()
            .id("transactional-dev".to_string())
            .name("My Dev Transactional Stream".to_string())
            .description("This is my second transactional stream")
            .message_stream_type(MessageStreamType::Transactional)
            .subscription_management_configuration(SubscriptionManagementConfiguration {
                unsubscribe_handling_type: UnsubscribeHandlingType::None,
            })
            .build();

        let resp = req.execute(&client).await.expect("json decode");

        assert_eq!(resp.id, "transactional-dev");
        assert_eq!(resp.message_stream_type, MessageStreamType::Transactional);
    }
}
