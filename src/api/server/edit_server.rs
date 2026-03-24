use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::api::server::{GetServerResponse, ServerIdOrName};
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct EditServerRequest {
    #[serde(skip)]
    pub server_id: ServerIdOrName,
    #[builder(setter(into))]
    pub name: String,
}

pub type EditServerResponse = GetServerResponse;

impl Endpoint for EditServerRequest {
    type Request = EditServerRequest;
    type Response = EditServerResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/servers/{}", self.server_id).into()
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
    async fn edit_server_puts_to_servers_id_or_name() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("PUT", "/servers/12345")).respond_with(
                json_encoded(json!({
                    "ID": 12345,
                    "Name": "New Name",
                    "ApiTokens": ["token"]
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = EditServerRequest::builder()
            .server_id(ServerIdOrName::ServerId(12345))
            .name("New Name")
            .build();

        assert_eq!(req.method(), http::Method::PUT);
        assert_eq!(req.endpoint(), "/servers/12345");
        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            json!({
                "Name": "New Name"
            })
        );

        req.execute(&client)
            .await
            .expect("Should decode edit server response");
    }

    #[test]
    fn edit_server_endpoint_supports_server_name() {
        let req = EditServerRequest::builder()
            .server_id(ServerIdOrName::ServerName("staging".into()))
            .name("New Name")
            .build();

        assert_eq!(req.endpoint(), "/servers/staging");
    }
}
