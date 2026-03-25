//! You'll find in email sending related endpoints.

pub use crate::api::server::ServerIdOrName;
use crate::api::types::id_type;
pub use create_webhook::*;
pub use delete_webhook::*;
pub use edit_webhook::*;
pub use get_webhook::*;
pub use list_webhooks::*;

id_type!(pub WebhookId);

mod create_webhook;
mod delete_webhook;
mod edit_webhook;
mod get_webhook;
mod list_webhooks;
