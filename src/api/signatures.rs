//! Sender signatures API endpoints.

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

mod create_signature;
mod delete_signature;
mod edit_signature;
mod get_signature;
mod list_signatures;
mod resend_signature_confirmation;

pub use create_signature::*;
pub use delete_signature::*;
pub use edit_signature::*;
pub use get_signature::*;
pub use list_signatures::*;
pub use resend_signature_confirmation::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SenderSignature {
    pub domain: String,
    pub email_address: String,
    pub reply_to_email_address: Option<String>,
    pub name: String,
    pub confirmed: bool,
    #[serde(rename = "SPFVerified")]
    pub spf_verified: Option<bool>,
    #[serde(rename = "SPFHost")]
    pub spf_host: Option<String>,
    #[serde(rename = "SPFTextValue")]
    pub spf_text_value: Option<String>,
    #[serde(rename = "DKIMVerified")]
    pub dkim_verified: bool,
    #[serde(rename = "WeakDKIM")]
    pub weak_dkim: bool,
    #[serde(rename = "DKIMHost")]
    pub dkim_host: Option<String>,
    #[serde(rename = "DKIMTextValue")]
    pub dkim_text_value: Option<String>,
    #[serde(rename = "DKIMPendingHost")]
    pub dkim_pending_host: Option<String>,
    #[serde(rename = "DKIMPendingTextValue")]
    pub dkim_pending_text_value: Option<String>,
    #[serde(rename = "DKIMRevokedHost")]
    pub dkim_revoked_host: Option<String>,
    #[serde(rename = "DKIMRevokedTextValue")]
    pub dkim_revoked_text_value: Option<String>,
    pub safe_to_remove_revoked_key_from_dns: Option<bool>,
    #[serde(rename = "DKIMUpdateStatus")]
    pub dkim_update_status: Option<String>,
    pub return_path_domain: Option<String>,
    pub return_path_domain_verified: Option<bool>,
    pub return_path_domain_cname_value: Option<String>,
    #[serde(rename = "ID")]
    pub id: isize,
    pub confirmation_personal_note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SenderSignatureSummary {
    pub domain: String,
    pub email_address: String,
    pub reply_to_email_address: Option<String>,
    pub name: String,
    pub confirmed: bool,
    #[serde(rename = "ID")]
    pub id: isize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BasicApiResponse {
    pub error_code: isize,
    pub message: String,
}

pub(crate) fn paginated_endpoint(path: &str, count: isize, offset: isize) -> Cow<'static, str> {
    format!("{path}?count={count}&offset={offset}").into()
}
