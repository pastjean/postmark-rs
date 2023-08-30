use crate::{api::Body, Endpoint};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use typed_builder::TypedBuilder;

use super::*;

/// Edit an existing e-mail template. The `id` field is used to identify the template
/// to be edited; this field will not impose any changes on the template. Other fields,
/// such as `alias` and `subject`, will change the values in the template. 
///
/// ```
/// use postmark::api::{Body, templates::{EditTemplateRequest, TemplateIdOrAlias}};
/// let req = EditTemplateRequest::builder()
///   .id(TemplateIdOrAlias::Alias(String::from("old-alias")))
///   .name("New template name")
///   .alias("new-alias")
///   .subject("Greetings!")
///   .body(Body::text("Hi, this is me!".to_string()))
///   .build();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct EditTemplateRequest {

    /// ID of template or template alias. This id or alias is used to identify the
    /// correct template to edit.
    #[serde(skip)]
    pub id: TemplateIdOrAlias,

    /// Name of template.
    #[builder(setter(into))]
    pub name: String,

    /// An optional string you can provide to identify this template (if creating
    /// a standard template). Allowed characters are numbers, ASCII letters, and
    /// ‘.’, ‘-’, ‘_’ characters, and the string has to start with a letter. This
    /// field will set or change the alias whereas the id field using the alias
    /// option will identify template by its current alias. 
    #[builder(default, setter(into, strip_option))]
    pub alias: Option<String>,

    /// The body of the message mau come in either or both of two types, HtmlBody or
    /// TextBody.
    ///
    /// HtmlBody is required if TextBody is not specified. See our template language
    /// documentation for more information on the [syntax for this field]
    /// (https://postmarkapp.com/support/article/1077-template-syntax). A content
    /// placeholder is required to be present for a layout template, and can be
    /// placed only once in the HtmlBody.
    ///
    /// TextBody is required if HtmlBody is not specified. A content
    /// placeholder is required to be present for a layout template, and can be
    /// placed only once in the TextBody.
    #[serde(flatten)]
    pub body: Body,

    /// The content to use for the Subject when this template is used to send email.
    /// Subject is only required on standard templates. See our template language
    /// documentation for more information on the [syntax for this field]
    /// (https://postmarkapp.com/support/article/1077-template-syntax). Subjects are
    ///  not allowed for layout templates and will result in an API error.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub subject: Option<String>,

    /// An optional string to specify which Layout Template to use (via layout alias)
    /// for an existing Layout Template when creating a standard template. Allowed
    /// characters are numbers, ASCII letters, and ‘.’, ‘-’, ‘_’ characters, and the
    /// string has to start with a letter.The API will throw an error if LayoutTemplate
    /// is present and the template type is a Layout. This field can also be set to
    /// null by using an empty string "".
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub layout_template: Option<String>,
}

/// Response for the [`CreateTemplateRequest`] Endpoint.
///
/// On a success all fields will be filled, `error_code` will be 0 and
/// message "OK".
/// On a failure Option fields will be empty and details will be held
/// in error_code and message.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EditTemplateResponse {
    /// ID of template
    pub template_id: isize,
    /// Name of template
    pub name: String,
    /// Indicates that this template may be used for sending email.
    pub active: bool,
    /// Template alias (or None if not specified).
    pub alias: Option<String>,
}

impl Endpoint for EditTemplateRequest {
    type Request = EditTemplateRequest;
    type Response = EditTemplateResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!{"/templates/{}", self.id}.into()
    }

    fn method(&self) -> http::Method {
        http::Method::PUT
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

    use super::*;
    use crate::reqwest::PostmarkClient;
    use crate::Query;

    const NAME: &str = "Onboarding Email";
    const ALIAS: &str = "my-template-alias";
    const TEXT_BODY: &str = "Welcome, {{name}}, you are a Postmark user.";
    const HTML_BODY: &str =
        "<html><body><strong>Welcome</strong>, {{name}}, you are a Postmark user.</body></html>";
    const SUBJ: &str = "Welcome to Postmark!";
    const LAYOUT_TEMPL: &str = "my-layout";

    #[tokio::test]
    pub async fn edit_template_test_with_text() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("PUT", "/templates/12345")).respond_with(
                json_encoded(json!({
                    "TemplateId": 12345,
                    "Name": NAME,
                    "Active": true,
                    "Alias": ALIAS,
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = EditTemplateRequest::builder()
            .id(TemplateIdOrAlias::TemplateId(12345))
            .name(NAME)
            .body(Body::text(TEXT_BODY.into()))
            .subject(SUBJ)
            .layout_template(LAYOUT_TEMPL)
            .build();

        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            json!({
                "Name": NAME,
                "Alias": serde_json::Value::Null,
                "TextBody": TEXT_BODY,
                "Subject": SUBJ,
                "LayoutTemplate": LAYOUT_TEMPL,
            })
        );

        req.execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");
    }

    #[tokio::test]
    pub async fn edit_template_test_with_html() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("PUT", "/templates/old-alias")).respond_with(
                json_encoded(json!({
                    "TemplateId": 12345,
                    "Name": NAME,
                    "Active": true,
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = EditTemplateRequest::builder()
            .id(TemplateIdOrAlias::Alias(String::from("old-alias")))
            .name(NAME)
            .alias(ALIAS)
            .body(Body::html(HTML_BODY.into()))
            .subject(SUBJ)
            .build();

        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            json!({
                "Name": NAME,
                "Alias": ALIAS,
                "HtmlBody": HTML_BODY,
                "Subject": SUBJ,
            })
        );

        req.execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");
    }

    #[tokio::test]
    pub async fn edit_template_test_with_text_and_html() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("PUT", "/templates/12345")).respond_with(
                json_encoded(json!({
                    "TemplateId": 12345,
                    "Name": NAME,
                    "Active": true,
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = EditTemplateRequest::builder()
            .id(TemplateIdOrAlias::TemplateId(12345))
            .name(NAME)
            .body(Body::html_and_text(HTML_BODY.into(), TEXT_BODY.into()))
            .subject(SUBJ)
            .build();

        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            json!({
                "Name": NAME,
                "Alias": serde_json::Value::Null,
                "HtmlBody": HTML_BODY,
                "TextBody": TEXT_BODY,
                "Subject": SUBJ,
            })
        );

        req.execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");
    }
}

