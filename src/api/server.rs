//! You'll find in email sending related endpoints.
use std::fmt;

use serde::{Deserialize, Serialize};

pub use create_server::*;

mod create_server;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServerIdOrName {
    ServerId(isize),
    ServerName(String),
}

impl fmt::Display for ServerIdOrName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServerIdOrName::ServerId(id) => write!(f, "{}", id),
            ServerIdOrName::ServerName(alias) => write!(f, "{}", alias),
        }
    }
}
