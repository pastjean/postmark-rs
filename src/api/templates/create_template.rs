use crate::{api::Body, Endpoint};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use typed_builder::TypedBuilder;

/// Create a new e-mail template
///
/// ```
/// use postmark::api::{Body, templates::{CreateTemplateRequest, TemplateType}};
/// let req = CreateTemplateRequest::builder()
///   .name("A great template")
///   .alias("Another great template name")
///   .template_type(TemplateType::Standard)
///   .body(Body::text("Hi, this is me!".to_string()))
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

    /// Available when creating a template. To set if a template is standard template
    /// or layout template. Possible options: Standard or Layout. Defaults to Standard.
    /// After creation, it's not possible to change a template type.
    #[serde(rename = "TemplateType", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub template_type: Option<TemplateType>,

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TemplateType {
    Standard,
    Layout,
}

impl Default for TemplateType {
    fn default() -> Self {
        TemplateType::Standard
    }
}

/// Response for the [`EditTemplateRequest`] Endpoint.
///
/// On a success all fields will be filled, `error_code` will be 0 and
/// message "OK".
/// On a failure Option fields will be empty and details will be held
/// in error_code and message.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateTemplateResponse {
    /// ID of template
    #[serde(rename = "TemplateID")]
    pub template_id: String,
    /// Name of template
    pub name: String,
    /// Indicates that this template may be used for sending email.
    pub active: bool,
    /// Template alias (or None if not specified).
    pub alias: Option<String>,
    /// Type of template. Possible options: Standard or Layout.
    pub template_type: TemplateType,
    /// Alias of layout used.
    pub layout_template: Option<String>,
}

impl Endpoint for CreateTemplateRequest {
    type Request = CreateTemplateRequest;
    type Response = CreateTemplateResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "/templates".into()
    }

    fn body(&self) -> &Self::Request {
        self
    }
}
