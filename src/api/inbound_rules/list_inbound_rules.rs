use std::borrow::Cow;

use serde::Serialize;

use crate::api::inbound_rules::ListInboundRulesResponse;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct ListInboundRulesRequest;

impl Endpoint for ListInboundRulesRequest {
    type Request = ListInboundRulesRequest;
    type Response = ListInboundRulesResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "/triggers/inboundrules".into()
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
    fn list_inbound_rules_uses_get_path() {
        let req = ListInboundRulesRequest {};

        assert_eq!(req.method(), http::Method::GET);
        assert_eq!(req.endpoint(), "/triggers/inboundrules");
    }
}
