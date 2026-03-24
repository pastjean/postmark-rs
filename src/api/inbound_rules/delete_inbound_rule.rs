use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::api::inbound_rules::InboundRuleActionResponse;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
pub struct DeleteInboundRuleRequest {
    #[serde(skip)]
    pub trigger_id: isize,
}

impl Endpoint for DeleteInboundRuleRequest {
    type Request = DeleteInboundRuleRequest;
    type Response = InboundRuleActionResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/triggers/inboundrules/{}", self.trigger_id).into()
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
    fn delete_inbound_rule_uses_delete_path() {
        let req = DeleteInboundRuleRequest::builder().trigger_id(123).build();

        assert_eq!(req.method(), http::Method::DELETE);
        assert_eq!(req.endpoint(), "/triggers/inboundrules/123");
    }
}
