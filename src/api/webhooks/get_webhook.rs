use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::api::webhooks::Webhook;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
pub struct GetWebhookRequest {
    #[serde(skip)]
    pub webhook_id: isize,
}

impl Endpoint for GetWebhookRequest {
    type Request = GetWebhookRequest;
    type Response = Webhook;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/webhooks/{}", self.webhook_id).into()
    }

    fn body(&self) -> &Self::Request {
        self
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_webhook_uses_get_path() {
        let req = GetWebhookRequest::builder().webhook_id(77).build();

        assert_eq!(req.method(), http::Method::GET);
        assert_eq!(req.endpoint(), "/webhooks/77");
    }
}
