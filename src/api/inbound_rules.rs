//! You'll find in inbound rules API related endpoints.

use serde::{Deserialize, Serialize};

mod create_inbound_rule;
mod delete_inbound_rule;
mod list_inbound_rules;

pub use create_inbound_rule::*;
pub use delete_inbound_rule::*;
pub use list_inbound_rules::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InboundRule {
    #[serde(rename = "ID")]
    pub id: isize,
    pub rule: String,
    pub forward_to: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListInboundRulesResponse {
    pub inbound_rules: Vec<InboundRule>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InboundRuleActionResponse {
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<isize>,
    pub error_code: i64,
    pub message: String,
}
