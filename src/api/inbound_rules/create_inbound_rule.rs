use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::api::inbound_rules::InboundRuleActionResponse;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct CreateInboundRuleRequest {
    #[builder(setter(into))]
    pub rule: String,
    #[builder(setter(into))]
    pub forward_to: String,
}

impl Endpoint for CreateInboundRuleRequest {
    type Request = CreateInboundRuleRequest;
    type Response = InboundRuleActionResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "/triggers/inboundrules".into()
    }

    fn body(&self) -> &Self::Request {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn create_inbound_rule_uses_post_path_and_pascal_case_body() {
        let req = CreateInboundRuleRequest::builder()
            .rule("has:attachment")
            .forward_to("https://example.com/inbound")
            .build();

        assert_eq!(req.method(), http::Method::POST);
        assert_eq!(req.endpoint(), "/triggers/inboundrules");
        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            json!({
                "Rule": "has:attachment",
                "ForwardTo": "https://example.com/inbound"
            })
        );
    }
}
