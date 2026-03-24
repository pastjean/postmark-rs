use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::api::webhooks::Webhook;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct EditWebhookRequest {
    #[serde(skip)]
    pub webhook_id: isize,
    #[builder(setter(into))]
    pub url: String,
    #[builder(setter(into))]
    pub message_stream: String,
}

impl Endpoint for EditWebhookRequest {
    type Request = EditWebhookRequest;
    type Response = Webhook;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/webhooks/{}", self.webhook_id).into()
    }

    fn body(&self) -> &Self::Request {
        self
    }

    fn method(&self) -> http::Method {
        http::Method::PUT
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn edit_webhook_uses_put_path() {
        let req = EditWebhookRequest::builder()
            .webhook_id(77)
            .url("https://example.com/webhook")
            .message_stream("outbound")
            .build();

        assert_eq!(req.method(), http::Method::PUT);
        assert_eq!(req.endpoint(), "/webhooks/77");
        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            json!({
                "Url": "https://example.com/webhook",
                "MessageStream": "outbound"
            })
        );
    }
}
