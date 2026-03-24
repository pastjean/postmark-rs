use std::borrow::Cow;

use crate::Endpoint;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteInboundRuleTriggerRequest {
    #[serde(skip)]
    pub trigger_id: isize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteInboundRuleTriggerResponse {
    pub error_code: isize,
    pub message: String,
}

impl Endpoint for DeleteInboundRuleTriggerRequest {
    type Request = DeleteInboundRuleTriggerRequest;
    type Response = DeleteInboundRuleTriggerResponse;

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
    use httptest::matchers::request;
    use httptest::{Expectation, Server, responders::*};
    use serde_json::json;

    use crate::Query;
    use crate::reqwest::PostmarkClient;

    use super::*;

    #[tokio::test]
    async fn delete_inbound_rule_trigger() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("DELETE", "/triggers/inboundrules/15"))
                .respond_with(json_encoded(
                    json!({"ErrorCode": 0, "Message": "Rule removed."}),
                )),
        );
        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = DeleteInboundRuleTriggerRequest::builder()
            .trigger_id(15)
            .build();
        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.error_code, 0);
    }
}
