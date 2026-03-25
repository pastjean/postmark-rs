//! You'll find in templates sending related endpoints.
use crate::api::types::id_type;
use serde::{Deserialize, Serialize};
use std::fmt;

id_type!(pub TemplateId);

mod copy_templates;
mod create_template;
mod delete_template;
mod edit_template;
mod get_template;
mod list_templates;
mod validate_template;

pub use copy_templates::*;
pub use create_template::*;
pub use delete_template::*;
pub use edit_template::*;
pub use get_template::*;
pub use list_templates::*;
pub use validate_template::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum TemplateType {
    #[default]
    Standard,
    Layout,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum TemplateAction {
    #[default]
    Create,
    Edit,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TemplateIdOrAlias {
    TemplateId(TemplateId),
    Alias(String),
}

impl From<TemplateId> for TemplateIdOrAlias {
    fn from(value: TemplateId) -> Self {
        Self::TemplateId(value)
    }
}

impl From<i64> for TemplateIdOrAlias {
    fn from(value: i64) -> Self {
        Self::TemplateId(value.into())
    }
}

impl From<String> for TemplateIdOrAlias {
    fn from(value: String) -> Self {
        Self::Alias(value)
    }
}

impl From<&str> for TemplateIdOrAlias {
    fn from(value: &str) -> Self {
        Self::Alias(value.to_owned())
    }
}

impl fmt::Display for TemplateIdOrAlias {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TemplateIdOrAlias::TemplateId(id) => write!(f, "{}", id),
            TemplateIdOrAlias::Alias(alias) => write!(f, "{}", alias),
        }
    }
}
