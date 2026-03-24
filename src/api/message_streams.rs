//! Message stream management API endpoints.

use serde::{Deserialize, Serialize};
use std::fmt;

mod archive_message_stream;
mod create_message_stream;
mod delete_suppression;
mod edit_message_stream;
mod get_message_stream;
mod get_suppressions;
mod list_message_streams;
mod unarchive_message_stream;

pub use archive_message_stream::*;
pub use create_message_stream::*;
pub use delete_suppression::*;
pub use edit_message_stream::*;
pub use get_message_stream::*;
pub use get_suppressions::*;
pub use list_message_streams::*;
pub use unarchive_message_stream::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageStreamType {
    Inbound,
    Broadcasts,
    Transactional,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageStreamTypeFilter {
    All,
    Inbound,
    Broadcasts,
    Transactional,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UnsubscribeHandlingType {
    #[serde(rename = "none")]
    None,
    Postmark,
    Custom,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_management_configuration: Option<SubscriptionManagementConfiguration>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ArchiveMessageStreamResponse {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "ServerID")]
    pub server_id: isize,
    pub expected_purge_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SuppressionStatusType {
    #[default]
    Deleted,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StreamIdOrName {
    StreamId(String),
    StreamName(String),
}

impl fmt::Display for StreamIdOrName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StreamIdOrName::StreamId(id) => write!(f, "{}", id),
            StreamIdOrName::StreamName(name) => write!(f, "{}", name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StreamIdOrName;

    #[test]
    fn display_stream_name() {
        let stream = StreamIdOrName::StreamName("broadcasts-dev".to_string());

        assert_eq!(stream.to_string(), "broadcasts-dev");
    }
}
