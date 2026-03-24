//! You'll find in bounce sending related endpoints.
mod activate_bounce;
mod delivery_stats;
mod get_bounce;
mod get_bounce_dump;
mod get_bounces;
mod list_bounces;

pub use activate_bounce::*;
pub use delivery_stats::*;
pub use get_bounce::*;
pub use get_bounce_dump::*;
pub use get_bounces::*;
pub use list_bounces::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BounceInfo {
    #[serde(rename = "ID")]
    pub id: isize,
    #[serde(rename = "Type")]
    pub bounce_type: String,
    pub type_code: isize,
    pub name: String,
    pub tag: Option<String>,
    #[serde(rename = "MessageID")]
    pub message_id: String,
    pub description: String,
    pub details: String,
    pub email: String,
    pub bounced_at: String,
    pub dump_available: bool,
    pub inactive: bool,
    pub can_activate: bool,
    pub content: Option<String>,
    pub subject: Option<String>,
}
