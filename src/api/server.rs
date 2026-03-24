//! Server management API endpoints.

pub use create_server::*;
pub use delete_server::*;
pub use edit_server::*;
pub use get_server::*;
pub use list_servers::*;
use serde::{Deserialize, Serialize};
use std::fmt;

mod create_server;
mod delete_server;
mod edit_server;
mod get_server;
mod list_servers;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServerIdOrName {
    ServerId(isize),
    ServerName(String),
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

impl fmt::Display for ServerIdOrName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServerIdOrName::ServerId(id) => write!(f, "{}", id),
            ServerIdOrName::ServerName(name) => write!(f, "{}", name),
        }
    }
}
