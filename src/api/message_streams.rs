//! You'll find in templates sending related endpoints.

use serde::{Deserialize, Serialize};
use std::fmt;

mod create_suppression;
mod delete_suppression;
mod get_message_stream;
mod get_suppressions;
mod list_message_streams;

pub use create_suppression::*;
pub use delete_suppression::*;
pub use get_message_stream::*;
pub use get_suppressions::*;
pub use list_message_streams::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SuppressionStatusType {
    #[default]
    Deleted,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SuppressionCreateStatusType {
    #[default]
    Suppressed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StreamIdOrName {
    StreamId(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum MessageStreamType {
    #[default]
    Inbound,
    Broadcasts,
    Transactional,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum UnsubscribeHandlingType {
    #[default]
    None,
    Custom,
    Postmark,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SubscriptionManagementConfiguration {
    pub unsubscribe_handling_type: UnsubscribeHandlingType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MessageStream {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "ServerID")]
    pub server_id: isize,
    pub name: String,
    pub description: Option<String>,
    pub message_stream_type: MessageStreamType,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub archived_at: Option<String>,
    pub expected_purge_date: Option<String>,
    pub subscription_management_configuration: SubscriptionManagementConfiguration,
}

impl fmt::Display for StreamIdOrName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StreamIdOrName::StreamId(id) => write!(f, "{}", id),
        }
    }
}

impl fmt::Display for MessageStreamType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MessageStreamType::Inbound => write!(f, "Inbound"),
            MessageStreamType::Broadcasts => write!(f, "Broadcasts"),
            MessageStreamType::Transactional => write!(f, "Transactional"),
            MessageStreamType::All => write!(f, "All"),
        }
    }
}
