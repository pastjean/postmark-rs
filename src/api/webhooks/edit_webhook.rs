use std::borrow::Cow;

use crate::api::webhooks::{Webhook, WebhookHeader, WebhookHttpAuth, WebhookTriggers};
use crate::Endpoint;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct EditWebhookRequest {
    #[serde(skip)]
    pub id: isize,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_auth: Option<WebhookHttpAuth>,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_headers: Option<Vec<WebhookHeader>>,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<WebhookTriggers>,
}

impl Endpoint for EditWebhookRequest {
    type Request = EditWebhookRequest;
    type Response = Webhook;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/webhooks/{}", self.id).into()
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
    use httptest::matchers::request;
    use httptest::{responders::*, Expectation, Server};
    use serde_json::json;

    use crate::reqwest::PostmarkClient;
    use crate::Query;

    use super::*;

    #[tokio::test]
    pub async fn edit_webhook() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("PUT", "/webhooks/1234567")).respond_with(
                json_encoded(json!({
                    "ID": 1234567,
                    "Url": "https://www.example.com/webhooks",
                    "MessageStream": "outbound",
                    "HttpAuth": {"Username": "user", "Password": "pass"},
                    "HttpHeaders": [{"Name": "name", "Value": "value"}],
                    "Triggers": {
                        "Open": {"Enabled": true, "PostFirstOpenOnly": false},
                        "Click": {"Enabled": true},
                        "Delivery": {"Enabled": true},
                        "Bounce": {"Enabled": true, "IncludeContent": false},
                        "SpamComplaint": {"Enabled": true, "IncludeContent": false},
                        "SubscriptionChange": {"Enabled": false}
                    }
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = EditWebhookRequest::builder()
            .id(1234567)
            .url("https://www.example.com/webhooks")
            .build();

        let resp = req
            .execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");

        assert_eq!(resp.id, 1234567);
        assert_eq!(resp.url, "https://www.example.com/webhooks");
    }
}
