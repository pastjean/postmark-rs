//! You'll find in email sending related endpoints.

use crate::api::types::id_type;
pub use create_server::*;
pub use delete_server::*;
pub use edit_server::*;
pub use edit_server_by_id::*;
pub use get_current_server::*;
pub use get_server::*;
pub use list_servers::*;
use serde::{Deserialize, Serialize};
use std::fmt;

id_type!(pub ServerId);

mod create_server;
mod delete_server;
mod edit_server;
mod edit_server_by_id;
mod get_current_server;
mod get_server;
mod list_servers;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServerIdOrName {
    ServerId(ServerId),
    ServerName(String),
}

impl From<ServerId> for ServerIdOrName {
    fn from(value: ServerId) -> Self {
        Self::ServerId(value)
    }
}

impl From<i64> for ServerIdOrName {
    fn from(value: i64) -> Self {
        Self::ServerId(value.into())
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeliveryType {
    #[default]
    Live,
    Sandbox,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServerColor {
    #[default]
    Purple,
    Blue,
    Turquoise,
    Green,
    Red,
    Yellow,
    Grey,
    Orange,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Server {
    #[serde(rename = "ID")]
    pub id: ServerId,
    pub name: String,
    pub api_tokens: Vec<String>,
    #[serde(default)]
    pub smtp_api_activated: bool,
}

impl fmt::Display for ServerIdOrName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServerIdOrName::ServerId(id) => write!(f, "{}", id),
            ServerIdOrName::ServerName(name) => write!(f, "{}", name),
        }
    }
}
