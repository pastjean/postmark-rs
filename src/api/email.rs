//! You'll find in email sending related endpoints.
mod send_email;
mod send_email_batch;
mod send_email_with_template;

pub use send_email::*;
pub use send_email_batch::*;
pub use send_email_with_template::*;
