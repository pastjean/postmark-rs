//! You'll find in messages API related endpoints.

use serde::{Deserialize, Serialize};

mod bypass_blocked_inbound;
mod inbound_details;
mod inbound_search;
mod message_clicks;
mod message_opens;
mod outbound_details;
mod outbound_dump;
mod outbound_search;
mod retry_failed_inbound;
mod single_message_clicks;
mod single_message_opens;

pub use bypass_blocked_inbound::*;
pub use inbound_details::*;
pub use inbound_search::*;
pub use message_clicks::*;
pub use message_opens::*;
pub use outbound_details::*;
pub use outbound_dump::*;
pub use outbound_search::*;
pub use retry_failed_inbound::*;
pub use single_message_clicks::*;
pub use single_message_opens::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MessageSummary {
    #[serde(rename = "MessageID")]
    pub message_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OutboundMessageDetails {
    #[serde(rename = "MessageID")]
    pub message_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_stream: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InboundMessageDetails {
    #[serde(rename = "MessageID")]
    pub message_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailbox_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_stream: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MessageDump {
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MessageActionResponse {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MessageOpen {
    #[serde(rename = "MessageID")]
    pub message_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MessageClick {
    #[serde(rename = "MessageID")]
    pub message_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
}
