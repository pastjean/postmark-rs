use std::borrow::Cow;

use serde::Serialize;

use crate::api::webhooks::ListWebhooksResponse;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct ListWebhooksRequest;

impl Endpoint for ListWebhooksRequest {
    type Request = ListWebhooksRequest;
    type Response = ListWebhooksResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "/webhooks".into()
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
    fn list_webhooks_uses_get_path() {
        let req = ListWebhooksRequest {};

        assert_eq!(req.method(), http::Method::GET);
        assert_eq!(req.endpoint(), "/webhooks");
    }
}
