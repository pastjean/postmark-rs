use std::borrow::Cow;

use crate::Endpoint;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct ListInboundRuleTriggersRequest {
    #[serde(skip)]
    pub count: isize,
    #[serde(skip)]
    pub offset: isize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InboundRule {
    #[serde(rename = "ID")]
    pub id: isize,
    pub rule: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListInboundRuleTriggersResponse {
    pub total_count: isize,
    pub inbound_rules: Vec<InboundRule>,
}

impl Endpoint for ListInboundRuleTriggersRequest {
    type Request = ListInboundRuleTriggersRequest;
    type Response = ListInboundRuleTriggersResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "/triggers/inboundrules?count={}&offset={}",
            self.count, self.offset
        )
        .into()
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
    use httptest::matchers::request;
    use httptest::{responders::*, Expectation, Server};
    use serde_json::json;

    use crate::reqwest::PostmarkClient;
    use crate::Query;

    use super::*;

    #[tokio::test]
    async fn list_inbound_rule_triggers() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/triggers/inboundrules"))
                .respond_with(json_encoded(json!({
                    "TotalCount": 1,
                    "InboundRules": [{"ID": 3, "Rule": "someone@example.com"}]
                }))),
        );
        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = ListInboundRuleTriggersRequest::builder()
            .count(50)
            .offset(0)
            .build();
        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.total_count, 1);
    }
}
