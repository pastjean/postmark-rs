use crate::{api::Body, Endpoint};
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
    #[serde(skip_serializing_if = "Option::is_none")]
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