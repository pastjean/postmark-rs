use std::borrow::Cow;

use super::send_email::{SendEmailRequest, SendEmailResponse};
use crate::Endpoint;

/// Send multiple emails at once
pub type SendEmailBatchRequest = Vec<SendEmailRequest>;
/// Response for [`SendEmailBatchRequest`]
pub type SendEmailBatchResponse = Vec<SendEmailResponse>;

impl Endpoint for SendEmailBatchRequest {
    type Request = SendEmailBatchRequest;
    type Response = SendEmailBatchResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "email/batch".into()
    }

    fn body(&self) -> &Self::Request {
        self
    }
}
