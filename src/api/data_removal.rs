//! Data Removal API endpoints.

mod create_data_removal;
mod get_data_removal_status;
use crate::api::types::id_type;

pub use create_data_removal::*;
pub use get_data_removal_status::*;

id_type!(pub DataRemovalId);
