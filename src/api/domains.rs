//! You'll find in domains API related endpoints.

use serde::{Deserialize, Serialize};

mod create_domain;
mod delete_domain;
mod edit_domain;
mod get_domain;
mod list_domains;
mod rotate_dkim;
mod verify_dkim;
mod verify_return_path;
mod verify_spf;

pub use create_domain::*;
pub use delete_domain::*;
pub use edit_domain::*;
pub use get_domain::*;
pub use list_domains::*;
pub use rotate_dkim::*;
pub use verify_dkim::*;
pub use verify_return_path::*;
pub use verify_spf::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Domain {
    #[serde(rename = "ID")]
    pub id: isize,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListDomainsResponse {
    pub total_count: isize,
    pub domains: Vec<Domain>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DomainActionResponse {
    pub error_code: i64,
    pub message: String,
}
