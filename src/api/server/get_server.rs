use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::Endpoint;
use crate::api::endpoint_with_path_segment;
use crate::api::server::{Server, ServerIdOrName};

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct GetServerRequest {
    #[builder(setter(into))]
    #[serde(skip)]
    pub server_id: ServerIdOrName,
}

impl Endpoint for GetServerRequest {
    type Request = GetServerRequest;
    type Response = Server;

    fn endpoint(&self) -> Cow<'static, str> {
        endpoint_with_path_segment("/servers", &self.server_id.to_string())
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

    const SERVER_ID: i64 = 123456;
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
            .server_id(ServerIdOrName::ServerId(SERVER_ID.into()))
            .build();

        req.execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");
    }
}
