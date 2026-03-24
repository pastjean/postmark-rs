use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::api::webhooks::WebhookActionResponse;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
pub struct DeleteWebhookRequest {
    #[serde(skip)]
    pub webhook_id: isize,
}

impl Endpoint for DeleteWebhookRequest {
    type Request = DeleteWebhookRequest;
    type Response = WebhookActionResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/webhooks/{}", self.webhook_id).into()
    }

    fn body(&self) -> &Self::Request {
        self
    }

    fn method(&self) -> http::Method {
        http::Method::DELETE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delete_webhook_uses_delete_path() {
        let req = DeleteWebhookRequest::builder().webhook_id(77).build();

        assert_eq!(req.method(), http::Method::DELETE);
        assert_eq!(req.endpoint(), "/webhooks/77");
    }
}
