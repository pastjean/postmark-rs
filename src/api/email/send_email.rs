use crate::Endpoint;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap};
use typed_builder::TypedBuilder;

/// Send a Single email
///
/// ```
/// # use postmark::api::email::{SendEmailRequest, Body};
/// let req = SendEmailRequest::builder()
///   .from("me@example.com")
///   .to("you@example.com")
///   .body(Body::Text("Hi, this is me!".to_string()))
///   .build();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct SendEmailRequest {
    /// The sender email address. Must have a registered and confirmed Sender Signature.
    /// To include a name, use the format `Full Name <sender@domain.com>` for the address.
    #[builder(setter(into))]
    pub from: String,

    /// Recipient email address. Multiple addresses are comma separated. Max 50.
    #[builder(setter(into))]
    pub to: String,

    /// The body of the message
    #[serde(flatten)]
    pub body: Body,

    /// Cc recipient email address. Multiple addresses are comma separated. Max 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub cc: Option<String>,

    /// Bcc recipient email address. Multiple addresses are comma separated. Max 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub bcc: Option<String>,

    /// Email subject
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub subject: Option<String>,

    /// Email tag that allows you to categorize outgoing emails and get detailed statistics.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub tag: Option<String>,

    /// Reply To override email address. Defaults to the Reply To set in the sender signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub reply_to: Option<String>,

    /// List of custom headers to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub headers: Option<Vec<Header>>,

    /// Activate open tracking for this email.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub track_opens: Option<bool>,

    /// Activate link tracking for links in the HTML or Text bodies of this email.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub track_links: Option<TrackLink>,

    /// List of attachments
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub attachments: Option<Vec<Attachment>>,

    /// Custom metadata key/value pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub metadata: Option<HashMap<String, String>>,

    /// Set message stream ID that's used for sending. If not provided, message will default to the "outbound" transactional stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub message_stream: Option<String>,
}

/// The body of a email message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Body {
    #[serde(rename = "TextBody")]
    Text(String),
    #[serde(rename = "HtmlBody")]
    Html(String),
    HtmlAndText {
        #[serde(rename = "HtmlBody")]
        html: String,
        #[serde(rename = "TextBody")]
        text: String,
    },
}

impl Default for Body {
    fn default() -> Self {
        Body::Text("".into())
    }
}

/// A custom headers to include in a email.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Header {
    pub name: String,
    pub value: String,
}

/// And attachment to an email.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Attachment {
    pub name: String,
    pub content: String,
    pub content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_id: Option<String>,
}

/// Activate link tracking for links in the HTML or Text bodies of this email.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrackLink {
    None,
    HtmlAndText,
    HtmlOnly,
    TextOnly,
}

impl Default for TrackLink {
    fn default() -> Self {
        Self::None
    }
}

/// Response for the [`SendEmailRequest`] Endpoint.
///
/// On a success all fields will be filled, `error_code` will be 0 and
/// message "OK".
/// On a failure Option fields will be empty and details will be held
/// in error_code and message.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SendEmailResponse {
    pub to: Option<String>,
    pub submitted_at: Option<String>,
    #[serde(rename = "MessageID")]
    pub message_id: Option<String>,
    pub error_code: i64,
    pub message: String,
}

impl Endpoint for SendEmailRequest {
    type Request = SendEmailRequest;
    type Response = SendEmailResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "/email".into()
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

    use super::{Body, SendEmailRequest};
    use crate::reqwest::PostmarkClient;
    use crate::Query;

    #[tokio::test]
    pub async fn send_email_test() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("POST", "/email")).respond_with(
                json_encoded(json!({
                    "To": "receiver@example.com",
                    "SubmittedAt": "2014-02-17T07:25:01.4178645-05:00",
                    "MessageID": "0a129aee-e1cd-480d-b08d-4f48548ff48d",
                    "ErrorCode": 0,
                    "Message": "OK"
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = SendEmailRequest::builder()
            .from("pa@example.com")
            .to("mathieu@example.com")
            .body(Body::Text("hello matt".into()))
            .subject("hello")
            .build();

        req.execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");
    }

    #[tokio::test]
    pub async fn send_email_test_should_not_error_on_postmark_error() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("POST", "/email")).respond_with(
                json_encoded(json!({
                    "ErrorCode": 406,
                    "Message": "You tried to send to a recipient that has been marked as inactive. Found inactive addresses: example@example.com. Inactive recipients are ones that have generated a hard bounce, a spam complaint, or a manual suppression. "                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = SendEmailRequest::builder()
            .from("pa@example.com")
            .to("mathieu@example.com")
            .body(Body::Text("hello matt".into()))
            .subject("hello")
            .build();

        req.execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");
    }
}
