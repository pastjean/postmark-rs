use std::borrow::Cow;

use crate::Endpoint;
use crate::api::endpoint_with_query;
use crate::api::webhooks::WebhookId;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use url::form_urlencoded::Serializer;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct ListWebhooksRequest {
    #[serde(skip)]
    #[builder(default, setter(into, strip_option))]
    pub message_stream: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListWebhooksResponse {
    pub webhooks: Vec<Webhook>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Webhook {
    #[serde(rename = "ID")]
    pub webhook_id: WebhookId,
    pub url: String,
    pub message_stream: String,
    pub http_auth: Option<WebhookHttpAuth>,
    pub http_headers: Option<Vec<WebhookHeader>>,
    pub triggers: WebhookTriggers,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WebhookHttpAuth {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WebhookHeader {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WebhookTriggers {
    pub open: OpenTrigger,
    pub click: TriggerConfig,
    pub delivery: TriggerConfig,
    pub bounce: IncludeContentTrigger,
    pub spam_complaint: IncludeContentTrigger,
    pub subscription_change: TriggerConfig,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OpenTrigger {
    pub enabled: bool,
    pub post_first_open_only: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TriggerConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IncludeContentTrigger {
    pub enabled: bool,
    pub include_content: bool,
}

impl Endpoint for ListWebhooksRequest {
    type Request = ListWebhooksRequest;
    type Response = ListWebhooksResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        let mut serializer = Serializer::new(String::new());
        if let Some(message_stream) = self.message_stream.as_deref() {
            serializer.append_pair("MessageStream", message_stream);
        }

        let query = serializer.finish();
        endpoint_with_query("/webhooks", query)
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
    pub async fn list_webhooks() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/webhooks")).respond_with(
                json_encoded(json!({
                    "Webhooks": [
                        {
                            "ID": 1234567,
                            "Url": "https://www.example.com/webhook-test-tracking",
                            "MessageStream": "outbound",
                            "HttpAuth": {
                                "Username": "user",
                                "Password": "pass"
                            },
                            "HttpHeaders": [
                                {
                                    "Name": "name",
                                    "Value": "value"
                                }
                            ],
                            "Triggers": {
                                "Open": {
                                    "Enabled": true,
                                    "PostFirstOpenOnly": false
                                },
                                "Click": {"Enabled": true},
                                "Delivery": {"Enabled": true},
                                "Bounce": {"Enabled": false, "IncludeContent": false},
                                "SpamComplaint": {"Enabled": false, "IncludeContent": false},
                                "SubscriptionChange": {"Enabled": true}
                            }
                        }
                    ]
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = ListWebhooksRequest::builder()
            .message_stream("outbound")
            .build();

        assert_eq!(req.endpoint(), "/webhooks?MessageStream=outbound");

        let resp = req
            .execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");

        assert_eq!(resp.webhooks.len(), 1);
        assert_eq!(resp.webhooks[0].webhook_id, 1234567);
        assert_eq!(resp.webhooks[0].message_stream, "outbound");
    }
}
