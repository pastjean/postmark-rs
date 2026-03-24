//! You'll find in webhook related endpoints.

pub use create_webhook::*;
pub use delete_webhook::*;
pub use edit_webhook::*;
pub use get_webhook::*;
pub use list_webhooks::*;
use serde::{Deserialize, Serialize};

mod create_webhook;
mod delete_webhook;
mod edit_webhook;
mod get_webhook;
mod list_webhooks;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Webhook {
    #[serde(rename = "ID")]
    pub webhook_id: isize,
    pub url: String,
    pub message_stream: Option<String>,
    pub triggers: Option<Triggers>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListWebhooksResponse {
    pub webhooks: Vec<Webhook>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WebhookActionResponse {
    pub error_code: i64,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Triggers {
    pub subscription_change: TriggerConfig,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct TriggerConfig {
    pub enabled: bool,
}
