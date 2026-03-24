use std::borrow::Cow;

use crate::Endpoint;
use crate::api::webhooks::{Webhook, WebhookId};
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct GetWebhookRequest {
    #[builder(setter(into))]
    #[serde(skip)]
    pub id: WebhookId,
}

impl Endpoint for GetWebhookRequest {
    type Request = GetWebhookRequest;
    type Response = Webhook;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/webhooks/{}", self.id).into()
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
    use httptest::{Expectation, Server, responders::*};
    use serde_json::json;

    use crate::Query;
    use crate::reqwest::PostmarkClient;

    use super::*;

    #[tokio::test]
    pub async fn get_webhook() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/webhooks/1234567")).respond_with(
                json_encoded(json!({
                    "ID": 1234567,
                    "Url": "https://www.example.com/webhook-test-tracking",
                    "MessageStream": "outbound",
                    "HttpAuth": {
                        "Username": "user",
                        "Password": "pass"
                    },
                    "HttpHeaders": [{"Name": "name", "Value": "value"}],
                    "Triggers": {
                        "Open": {"Enabled": true, "PostFirstOpenOnly": false},
                        "Click": {"Enabled": true},
                        "Delivery": {"Enabled": true},
                        "Bounce": {"Enabled": false, "IncludeContent": false},
                        "SpamComplaint": {"Enabled": false, "IncludeContent": false},
                        "SubscriptionChange": {"Enabled": true}
                    }
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = GetWebhookRequest::builder().id(1234567).build();

        let resp = req
            .execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");

        assert_eq!(resp.id, 1234567);
        assert_eq!(resp.url, "https://www.example.com/webhook-test-tracking");
    }
}
