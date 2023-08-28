//! You'll find in templates sending related endpoints.
use serde::{Deserialize, Serialize};
use std::fmt;

mod create_template;
mod edit_template;
mod get_template;
mod delete_template;

pub use create_template::*;
pub use edit_template::*;
pub use get_template::*;
pub use delete_template::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum TemplateType {
    #[default]
    Standard,
    Layout,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TemplateIdOrAlias {
    TemplateId(isize),
    Alias(String),
}

impl fmt::Display for TemplateIdOrAlias {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
