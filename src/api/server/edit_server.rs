use std::borrow::Cow;

use crate::Endpoint;
use crate::api::server::{DeliveryType, Server, ServerColor};
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct EditServerRequest {
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<ServerColor>,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_type: Option<DeliveryType>,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smtp_api_activated: Option<bool>,
}

impl Endpoint for EditServerRequest {
    type Request = EditServerRequest;
    type Response = Server;

    fn endpoint(&self) -> Cow<'static, str> {
        "/server".into()
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
    use httptest::{Expectation, Server as HttpServer, responders::*};
    use serde_json::json;

    use crate::Query;
    use crate::reqwest::PostmarkClient;

    use super::*;

    #[tokio::test]
    pub async fn edit_server() {
        let server = HttpServer::run();

        server.expect(
            Expectation::matching(request::method_path("PUT", "/server")).respond_with(
                json_encoded(json!({
                  "ID": 1,
                  "Name": "Staging Testing",
                  "ApiTokens": ["server token"],
                  "SmtpApiActivated": true
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = EditServerRequest::builder().name("Staging Testing").build();

        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.id, 1);
    }
}
