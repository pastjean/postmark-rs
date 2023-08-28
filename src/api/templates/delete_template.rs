use crate::{api::Body, Endpoint};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use typed_builder::TypedBuilder;

use super::*;

/// Create a new e-mail template
///
/// ```
/// use postmark::api::{Body, templates::{DeleteTemplateRequest, TemplateIdOrAlias}};
/// let req = DeleteTemplateRequest::builder()
///   .id(TemplateIdOrAlias::TemplateId(12345))
///   .build();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct DeleteTemplateRequest {
    /// ID of template or template alias
    pub id: TemplateIdOrAlias,
}

/// Response for the [`EditTemplateRequest`] Endpoint.
///
/// On a success all fields will be filled, `error_code` will be 0 and
/// message "OK".
/// On a failure Option fields will be empty and details will be held
/// in error_code and message.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteTemplateResponse {
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

impl Endpoint for DeleteTemplateRequest {
    type Request = DeleteTemplateRequest;
    type Response = DeleteTemplateResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/template/{}", self.id).into()
    }

    fn body(&self) -> &Self::Request {
        self
    }

    fn method(&self) -> http::Method {
        http::Method::DELETE
    }
}
