use std::borrow::Cow;

use crate::api::server::{DeliveryType, ServerColor};
use crate::Endpoint;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct CreateServerRequest {
    #[builder(setter(into))]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub color: Option<ServerColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub delivery_type: Option<DeliveryType>,
    #[builder(default = true)]
    pub smtp_api_activated: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateServerResponse {
    #[serde(rename = "ID")]
    pub id: isize,
    pub name: String,
    pub api_tokens: Vec<String>,
}

impl Endpoint for CreateServerRequest {
    type Request = CreateServerRequest;
    type Response = CreateServerResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "/servers".into()
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

    const NAME: &str = "Staging Testing";

    #[tokio::test]
    pub async fn create_new_server() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("POST", "/servers")).respond_with(
                json_encoded(json!({
                  "ID": 1,
                  "Name": "Staging Testing",
                  "ApiTokens": [
                    "server token"
                  ],
                  "Color": "red",
                  "SmtpApiActivated": true,
                  "RawEmailEnabled": false,
                  "DeliveryType": "Live",
                  "ServerLink": "https://postmarkapp.com/servers/1/streams",
                  "InboundAddress": "yourhash@inbound.postmarkapp.com",
                  "InboundHookUrl": "http://hooks.example.com/inbound",
                  "BounceHookUrl": "http://hooks.example.com/bounce",
                  "OpenHookUrl": "http://hooks.example.com/open",
                  "DeliveryHookUrl": "http://hooks.example.com/delivery",
                  "PostFirstOpenOnly": false,
                  "InboundDomain": "",
                  "InboundHash": "yourhash",
                  "InboundSpamThreshold": 5,
                  "TrackOpens": false,
                  "TrackLinks": "None",
                  "IncludeBounceContentInHook": true,
                  "ClickHookUrl": "http://hooks.example.com/click",
                  "EnableSmtpApiErrorHooks": false
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = CreateServerRequest::builder().name(NAME).build();

        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            json!({
                "Name": NAME,
                "SmtpApiActivated": true,
            })
        );

        req.execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");
    }
}
