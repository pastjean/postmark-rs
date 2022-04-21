use crate::Endpoint;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::{HashMap, BTreeMap}};
use typed_builder::TypedBuilder;

use super::send_email::{Header, Attachment, TrackLink, SendEmailResponse};

/// The template model. It is essentially a serde-serializable hashmap.
///
/// ```
/// # use postmark::api::email::TemplateModel;
/// let mut model = TemplateModel::new();
/// model.insert("name", "Ferris");
/// model.insert("favorite_food", ["algae", "seaweed", "shrimp", "cpp"]);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TemplateModel {
    model: HashMap<String, serde_json::Value>,
}

impl TemplateModel {
    fn new() -> Self { Self { model: HashMap::new() } }

    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Serialize
    {
        self.model.insert(key.into(), serde_json::to_value(value).unwrap());
    }

    pub fn remove<K>(&mut self, key: K)
    where
        K: Into<String>
    {
        self.model.remove(&key.into());
    }

    pub fn into_inner(self) -> HashMap<String, serde_json::Value> {
        self.model
    }
}

impl<K: Into<String>, V: Serialize> From<HashMap<K, V>> for TemplateModel {
    fn from(model: HashMap<K, V>) -> Self {
        Self { model: model.into_iter().map(|(k, v)| (k.into(), serde_json::to_value(v).unwrap())).collect() }
    }
}

impl<K: Into<String>, V: Serialize> From<BTreeMap<K, V>> for TemplateModel {
    fn from(model: BTreeMap<K, V>) -> Self {
        Self { model: model.into_iter().map(|(k, v)| (k.into(), serde_json::to_value(v).unwrap())).collect() }
    }
}

#[cfg(feature = "indexmap")]
impl<K: Into<String>, V: Serialize> From<indexmap::IndexMap<K, V>> for TemplateModel {
    fn from(model: indexmap::IndexMap<K, V>) -> Self {
        Self { model: model.into_iter().map(|(k, v)| (k.into(), serde_json::to_value(v).unwrap())).collect() }
    }
}

/// Send a Single email with a template.
///
/// ```
/// # use postmark::api::email::{SendEmailWithTemplateRequest, TemplateModel};
/// let mut model = TemplateModel::new();
/// model.insert("name", "Ferris");
/// 
/// let req = SendEmailWithTemplateRequest::builder()
///   .from("me@example.com")
///   .to("you@example.com")
///   .template_model(model)
///   .build();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct SendEmailWithTemplateRequest {
    /// The sender email address. Must have a registered and confirmed Sender Signature.
    /// To include a name, use the format "Full Name <sender@domain.com>" for the address.
    #[builder(setter(into))]
    pub from: String,

    /// Recipient email address. Multiple addresses are comma separated. Max 50.
    #[builder(setter(into))]
    pub to: String,

    /// The template ID. One of template_id or template_alias must be provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub template_id: Option<i64>,

    /// The template alias. One of template_id or template_alias must be provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub template_alias: Option<String>,

    /// The template model.
    #[serde(flatten)]
    #[builder(default, setter(into))]
    pub template_model: TemplateModel,

    /// Cc recipient email address. Multiple addresses are comma separated. Max 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub cc: Option<String>,

    /// Bcc recipient email address. Multiple addresses are comma separated. Max 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub bcc: Option<String>,

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

impl Endpoint for SendEmailWithTemplateRequest
{
    type Request = SendEmailWithTemplateRequest;
    type Response = SendEmailResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "/email/withTemplate".into()
    }

    fn body(&self) -> &Self::Request {
        self
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use httptest::matchers::request;
    use httptest::{responders::*, Expectation, Server};
    use serde_json::json;

    use super::{SendEmailWithTemplateRequest, TemplateModel};
    use crate::reqwest::PostmarkClient;
    use crate::Query;

    #[tokio::test]
    pub async fn send_email_test() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("POST", "/email/withTemplate")).respond_with(
                json_encoded(json!({
                    "To": "receiver@example.com",
                    "SubmittedAt": "2014-02-17T07:25:01.4178645-05:00",
                    "MessageID": "0a129aee-e1cd-480d-b08d-4f48548ff48d",
                    "ErrorCode": 0_i64,
                    "Message": "OK"
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let mut model = TemplateModel::new();
        model.insert("name", "Ferris");
        model.insert("favorite_food", ["algae", "seaweed", "shrimp", "cpp"]);

        let req = SendEmailWithTemplateRequest::builder()
            .from("pa@example.com")
            .to("mathieu@example.com")
            .template_alias("my_template".to_string())
            .template_model(model)
            .build();

        req.execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");
    }

    #[tokio::test]
    pub async fn send_email_test_should_not_error_on_postmark_error() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("POST", "/email/withTemplate")).respond_with(
                json_encoded(json!({
                    "ErrorCode": 406_i64,
                    "Message": "You tried to send to a recipient that has been marked as inactive. Found inactive addresses: example@example.com. Inactive recipients are ones that have generated a hard bounce, a spam complaint, or a manual suppression. "                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let mut nested_map = HashMap::new();
        nested_map.insert("code".to_string(), 123_i64);

        let mut template = HashMap::new();
        template.insert("nested".to_string(), nested_map);

        let req = SendEmailWithTemplateRequest::builder()
            .from("pa@example.com")
            .to("mathieu@example.com")
            .template_id(123456)
            .template_model(TemplateModel::from(template))
            .build();

        req.execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");
    }
}
