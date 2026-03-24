//! You'll find in data removals API related endpoints.

use serde::{Deserialize, Serialize};

mod create_data_removal;
mod get_data_removal_status;

pub use create_data_removal::*;
pub use get_data_removal_status::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataRemovalResponse {
    #[serde(rename = "ID")]
    pub id: String,
    pub status: Option<String>,
    pub error_code: Option<i64>,
    pub message: Option<String>,
}
