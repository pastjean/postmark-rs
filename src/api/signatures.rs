//! You'll find in signatures API related endpoints.

use serde::{Deserialize, Serialize};

mod create_signature;
mod delete_signature;
mod edit_signature;
mod get_signature;
mod list_signatures;
mod request_new_dkim;
mod resend_confirmation;
mod verify_spf;

pub use create_signature::*;
pub use delete_signature::*;
pub use edit_signature::*;
pub use get_signature::*;
pub use list_signatures::*;
pub use request_new_dkim::*;
pub use resend_confirmation::*;
pub use verify_spf::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Signature {
    #[serde(rename = "ID")]
    pub id: isize,
    pub name: String,
    pub email_address: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListSignaturesResponse {
    pub total_count: isize,
    #[serde(rename = "Senders")]
    pub signatures: Vec<Signature>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SignatureActionResponse {
    pub error_code: i64,
    pub message: String,
}
