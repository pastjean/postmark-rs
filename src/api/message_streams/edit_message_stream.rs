use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::api::message_streams::{
    MessageStream, StreamIdOrName, SubscriptionManagementConfiguration,
};
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct EditMessageStreamRequest {
    #[serde(skip)]
    pub stream_id: StreamIdOrName,
    #[builder(setter(into))]
    pub name: String,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_management_configuration: Option<SubscriptionManagementConfiguration>,
}

pub type EditMessageStreamResponse = MessageStream;

impl Endpoint for EditMessageStreamRequest {
    type Request = EditMessageStreamRequest;
    type Response = EditMessageStreamResponse;

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

    use crate::reqwest::PostmarkClient;
    use crate::Query;

    use super::*;

    #[tokio::test]
    async fn edit_message_stream_patches_stream_and_decodes_response() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path(
                "PATCH",
                "/message-streams/transactional-dev",
            ))
            .respond_with(json_encoded(json!({
                "ID": "transactional-dev",
                "ServerID": 12345,
                "Name": "Updated Dev Stream",
                "Description": "Updated description",
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

        let req = EditMessageStreamRequest::builder()
            .stream_id(StreamIdOrName::StreamId("transactional-dev".to_string()))
            .name("Updated Dev Stream")
            .description("Updated description")
            .build();

        assert_eq!(req.method(), http::Method::PATCH);
        assert_eq!(req.endpoint(), "/message-streams/transactional-dev");
        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            json!({
                "Name": "Updated Dev Stream",
                "Description": "Updated description"
            })
        );

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode edit message stream response");

        assert_eq!(resp.id, "transactional-dev");
        assert_eq!(resp.name, "Updated Dev Stream");
    }
}
