use crate::{api::Body, Endpoint};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use typed_builder::TypedBuilder;

/// Edit an existing e-mail template
///
/// ```
/// use postmark::api::{Body, templates::EditTemplateRequest};
/// let req = EditTemplateRequest::builder()
///   .name("Old template name")
///   .alias("New template alias")
///   .subject("Greetings!")
///   .body(Body::text("Hi, this is me!".to_string()))
///   .build();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct EditTemplateRequest {
    /// Name of template.
    #[builder(setter(into))]
    pub name: String,

    /// An optional string you can provide to identify this template (if creating
    /// a standard template). Allowed characters are numbers, ASCII letters, and
    /// ‘.’, ‘-’, ‘_’ characters, and the string has to start with a letter.
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
    //    #[serde(rename = "TemplateID")]
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
        "/templates/{templateIdOrAlias}".into()
    }

    fn method(&self) -> http::Method {
        http::Method::PUT
    }

    fn body(&self) -> &Self::Request {
        self
    }
}
