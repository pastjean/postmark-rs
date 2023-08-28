use crate::{api::Body, Endpoint};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use typed_builder::TypedBuilder;

use super::*;

/// Create a new e-mail template
///
/// ```
/// use postmark::api::{Body, templates::{GetTemplateRequest, TemplateIdOrAlias}};
/// let req = GetTemplateRequest::builder()
///   .id(TemplateIdOrAlias::TemplateId(12345))
///   .build();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct GetTemplateRequest {
    /// ID of template or template alias
    pub id: TemplateIdOrAlias,
}

// impl GetTemplateRequest {
//     fn template_id_or_alias(&self) -> String {
//         match self.template_id {
//             Some(id) => id.to_string(),
//             None => match &self.alias {
//                 Some(alias) => alias.to_string(),
//                 None => String::from(""),
//             }
//         }
//     }
// }

/// Response for the [`EditTemplateRequest`] Endpoint.
///
/// On a success all fields will be filled, will be 0 and
/// message "OK".
/// On a failure Option fields will be empty and details will be held
/// in error_code and message.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetTemplateResponse {
    /// ID of template
//    #[serde(rename = "TemplateID")]
    pub template_id: isize,
    /// Name of template
    pub name: String,
    /// The content to use for the Subject when this template is used to send email.
    pub subject: String,
    /// The content to use for the HtmlBody and/or TextBody when this template is
    /// used to send email.
    pub body: Body,
    /// The ID of the Server with which this template is associated.
    pub associated_server_id: isize,
    /// Indicates that this template may be used for sending email.
    pub active: bool,
    /// Template alias (or None if not specified).
    pub alias: Option<String>,
    /// Type of template. Possible options: Standard or Layout.
    pub template_type: TemplateType,
    /// Alias of layout used.
    pub layout_template: Option<String>,
}

impl Endpoint for GetTemplateRequest {
    type Request = GetTemplateRequest;
    type Response = GetTemplateResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/template/{}", self.id).into()
    }

    fn body(&self) -> &Self::Request {
        self
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
}