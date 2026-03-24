//! You'll find in email sending related endpoints.

pub use create_webhook::*;
pub use delete_webhook::*;
pub use edit_webhook::*;
pub use get_webhook::*;
pub use list_webhooks::*;
use serde::{Deserialize, Serialize};
use std::fmt;

mod create_webhook;
mod delete_webhook;
mod edit_webhook;
mod get_webhook;
mod list_webhooks;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServerIdOrName {
    ServerId(isize),
    ServerName(String),
}

impl fmt::Display for ServerIdOrName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServerIdOrName::ServerId(id) => write!(f, "{}", id),
            ServerIdOrName::ServerName(name) => write!(f, "{}", name),
        }
    }
}
