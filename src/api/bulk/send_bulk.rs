use std::borrow::Cow;
use std::collections::HashMap;

use crate::api::email::{Attachment, Header, TrackLink};
use crate::Endpoint;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct SendBulkEmailRequest {
    pub from: String,
    pub messages: Vec<BulkMessage>,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_body: Option<String>,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_body: Option<String>,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<isize>,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_alias: Option<String>,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_css: Option<bool>,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_stream: Option<String>,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_opens: Option<bool>,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_links: Option<TrackLink>,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<Header>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BulkMessage {
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_model: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<Header>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SendBulkEmailResponse {
    #[serde(rename = "ID")]
    pub id: String,
    pub status: String,
    pub submitted_at: String,
}

impl Endpoint for SendBulkEmailRequest {
    type Request = SendBulkEmailRequest;
    type Response = SendBulkEmailResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "/email/bulk".into()
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
    async fn send_bulk_email() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("POST", "/email/bulk")).respond_with(
                json_encoded(json!({
                    "ID": "f24af63c-533d-4b7a-ad65-4a7b3202d3a7",
                    "Status": "Accepted",
                    "SubmittedAt": "2024-03-17T07:25:01.4178645-05:00"
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = SendBulkEmailRequest::builder()
            .from("sender@example.com".to_string())
            .subject("This is a bulk email for {{FirstName}}")
            .text_body("Hi, {{FirstName}}")
            .message_stream("broadcast")
            .messages(vec![BulkMessage {
                to: "receiver1@example.com".to_string(),
                template_model: Some(json!({"FirstName":"Bob"})),
                ..Default::default()
            }])
            .build();

        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.status, "Accepted");
    }
}
