use std::borrow::Cow;

use crate::Endpoint;
use crate::api::endpoint_with_path_segment;
use crate::api::server::{DeliveryType, Server, ServerColor, ServerId};
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct EditServerByIdRequest {
    #[builder(setter(into))]
    #[serde(skip)]
    pub server_id: ServerId,
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

impl Endpoint for EditServerByIdRequest {
    type Request = EditServerByIdRequest;
    type Response = Server;

    fn endpoint(&self) -> Cow<'static, str> {
        endpoint_with_path_segment("/servers", &self.server_id.to_string())
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
    pub async fn edit_server_by_id() {
        let server = HttpServer::run();

        server.expect(
            Expectation::matching(request::method_path("PUT", "/servers/1")).respond_with(
                json_encoded(json!({
                    "ID": 1,
                    "Name": "Production 2",
                    "ApiTokens": ["server token"],
                    "SmtpApiActivated": true
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = EditServerByIdRequest::builder()
            .server_id(1)
            .name("Production 2")
            .build();

        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.id, 1);
        assert_eq!(resp.name, "Production 2");
    }
}
