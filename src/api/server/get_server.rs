use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::api::server::ServerIdOrName;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct GetServerRequest {
    #[serde(skip)]
    pub server_id: ServerIdOrName,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetServerResponse {
    #[serde(rename = "ID")]
    pub id: isize,
    pub name: String,
    pub api_tokens: Vec<String>,
}

impl Endpoint for GetServerRequest {
    type Request = GetServerRequest;
    type Response = GetServerResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/servers/{}", self.server_id).into()
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

    const SERVER_ID: isize = 123456;
    #[tokio::test]
    pub async fn get_server() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", format!("/servers/{SERVER_ID}")))
                .respond_with(json_encoded(json!({
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
                  "InboundSpamThreshold": 0,
                  "TrackOpens": false,
                  "TrackLinks": "None",
                  "IncludeBounceContentInHook": true,
                  "ClickHookUrl": "http://hooks.example.com/click",
                  "EnableSmtpApiErrorHooks": false
                }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = GetServerRequest::builder()
            .server_id(ServerIdOrName::ServerId(SERVER_ID))
            .build();

        print!("{}\n", req.endpoint());

        req.execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");
    }
}
