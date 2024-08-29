use std::borrow::Cow;

use crate::Endpoint;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct CreateWebhookRequest {
    #[builder(setter(into))]
    pub url: String,
    pub message_stream: String,
    pub triggers: Triggers,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateWebhookResponse {
    #[serde(rename = "ID")]
    pub webhook_id: isize,
    pub triggers: Triggers,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Triggers {
    pub subscription_change: TriggerConfig,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct TriggerConfig {
    pub enabled: bool,
}

impl Endpoint for CreateWebhookRequest {
    type Request = CreateWebhookRequest;
    type Response = CreateWebhookResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "/webhooks".into()
    }

    fn body(&self) -> &Self::Request {
        self
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
    const WEBHOOK_URL: &str = "http://www.example.com/webhook-test-tracking";
    #[tokio::test]
    pub async fn create_webhook() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("POST", "/webhooks")).respond_with(
                json_encoded(json!({
                    "ID": 1234567,
                    "Url": "http://www.example.com/webhook-test-tracking",
                    "MessageStream": "outbound",
                    "HttpAuth":{
                        "Username": "user",
                        "Password": "pass"
                    },
                    "HttpHeaders":[
                        {
                            "Name": "name",
                            "Value": "value"
                        }
                    ],
                    "Triggers": {
                        "Open":{
                            "Enabled": true,
                            "PostFirstOpenOnly": false
                        },
                        "Click":{
                            "Enabled": true
                        },
                        "Delivery":{
                            "Enabled": true
                        },
                        "Bounce":{
                            "Enabled": false,
                            "IncludeContent": false
                        },
                        "SpamComplaint":{
                            "Enabled": false,
                            "IncludeContent": false
                        },
                        "SubscriptionChange": {
                            "Enabled": true
                        }
                    }
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = CreateWebhookRequest::builder()
            .url(String::from(WEBHOOK_URL))
            .message_stream(String::from("broadcast"))
            .triggers(Triggers {
                subscription_change: TriggerConfig { enabled: true },
            })
            .build();

        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            json!({
                "Url": WEBHOOK_URL,
                "MessageStream": ("broadcast"),
                "Triggers": {
                "SubscriptionChange": {
                    "Enabled": true
            }
            }
            })
        );

        req.execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");
    }
}
