//! Inbound rules triggers API endpoints.

mod create_inbound_rule_trigger;
mod delete_inbound_rule_trigger;
mod list_inbound_rule_triggers;
use crate::api::types::id_type;

pub use create_inbound_rule_trigger::*;
pub use delete_inbound_rule_trigger::*;
pub use list_inbound_rule_triggers::*;

id_type!(pub InboundRuleTriggerId);
