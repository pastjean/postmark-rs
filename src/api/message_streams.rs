//! You'll find in templates sending related endpoints.

use serde::{Deserialize, Serialize};
use std::fmt;

mod create_suppression;
mod delete_suppression;
mod get_suppressions;

pub use create_suppression::*;
pub use delete_suppression::*;
pub use get_suppressions::*;

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

impl fmt::Display for StreamIdOrName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StreamIdOrName::StreamId(id) => write!(f, "{}", id),
        }
    }
}
