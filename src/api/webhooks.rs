//! You'll find in email sending related endpoints.

use crate::api::types::id_type;
pub use create_webhook::*;
pub use delete_webhook::*;
pub use edit_webhook::*;
pub use get_webhook::*;
pub use list_webhooks::*;
use serde::{Deserialize, Serialize};
use std::fmt;

id_type!(pub WebhookId);
id_type!(pub WebhookServerId);

mod create_webhook;
mod delete_webhook;
mod edit_webhook;
mod get_webhook;
mod list_webhooks;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServerIdOrName {
    ServerId(WebhookServerId),
    ServerName(String),
}

impl From<WebhookServerId> for ServerIdOrName {
    fn from(value: WebhookServerId) -> Self {
        Self::ServerId(value)
    }
}

impl From<i64> for ServerIdOrName {
    fn from(value: i64) -> Self {
        Self::ServerId(value.into())
    }
}

impl fmt::Display for ServerIdOrName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServerIdOrName::ServerId(id) => write!(f, "{}", id),
            ServerIdOrName::ServerName(name) => write!(f, "{}", name),
        }
    }
}
