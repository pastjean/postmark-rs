use crate::Endpoint;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap};
use typed_builder::TypedBuilder;

/// Send a Single email
///
/// ```
/// # use postmark::api::email::{SendEmailRequest, Body};
/// let req = CreateTemplateRequest::builder()
///   .from("me@example.com")
///   .to("you@example.com")
///   .body(Body::Text("Hi, this is me!".to_string()))
///   .build();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct CreateTemplateRequest {
    /// Name of template.
    #[builder(setter(into))]
    pub name: String,

    /// An optional string you can provide to identify this template (if creating 
    /// a standard template). Allowed characters are numbers, ASCII letters, and 
    /// ‘.’, ‘-’, ‘_’ characters, and the string has to start with a letter.
    #[builder(setter(into))]
    pub alias: String,

    /// The body of the message
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