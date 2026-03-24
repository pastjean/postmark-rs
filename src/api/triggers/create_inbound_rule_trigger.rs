use std::borrow::Cow;

use crate::Endpoint;
use crate::api::triggers::InboundRule;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct CreateInboundRuleTriggerRequest {
    pub rule: String,
}

impl Endpoint for CreateInboundRuleTriggerRequest {
    type Request = CreateInboundRuleTriggerRequest;
    type Response = InboundRule;

    fn endpoint(&self) -> Cow<'static, str> {
        "/triggers/inboundrules".into()
    }

    fn body(&self) -> &Self::Request {
        self
    }
}

#[cfg(test)]
mod tests {
    use httptest::matchers::request;
    use httptest::{Expectation, Server, responders::*};
    use serde_json::json;

    use crate::Query;
    use crate::reqwest::PostmarkClient;

    use super::*;

    #[tokio::test]
    async fn create_inbound_rule_trigger() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("POST", "/triggers/inboundrules"))
                .respond_with(json_encoded(
                    json!({"ID": 15, "Rule": "someone@example.com"}),
                )),
        );
        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = CreateInboundRuleTriggerRequest::builder()
            .rule("someone@example.com".to_string())
            .build();
        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.trigger_id, 15);
    }
}
