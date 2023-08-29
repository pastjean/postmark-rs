use crate::Endpoint;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use typed_builder::TypedBuilder;

use super::*;

/// Create a new e-mail template
///
/// ```
/// use postmark::api::{Body, templates::{DeleteTemplateRequest, TemplateIdOrAlias}};
/// let req = DeleteTemplateRequest::builder()
///   .id(TemplateIdOrAlias::TemplateId(12345))
///   .build();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct DeleteTemplateRequest {
    /// ID of template or template alias
    pub id: TemplateIdOrAlias,
}

/// Response for the [`DeleteTemplateRequest`] Endpoint.
///
/// On a success all fields will be filled, `error_code` will be 0 and
/// message "OK".
/// On a failure Option fields will be empty and details will be held
/// in error_code and message.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteTemplateResponse {
    /// [API Error codes]: https://postmarkapp.com/developer/api/overview#error-codes
    pub error_code: i64,
    /// Associated success or error message.
    pub message: String,
}

impl Endpoint for DeleteTemplateRequest {
    type Request = DeleteTemplateRequest;
    type Response = DeleteTemplateResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/templates/{}", self.id).into()
    }

    fn body(&self) -> &Self::Request {
        self
    }

    fn method(&self) -> http::Method {
        http::Method::DELETE
    }
}

#[cfg(test)]
mod tests {
    use httptest::matchers::request;
    use httptest::{responders::*, Expectation, Server};
    use serde_json::json;

    use super::*;
    use crate::reqwest::PostmarkClient;
    use crate::Query;

    const ALIAS: &str = "my-template-alias";

    #[tokio::test]
    pub async fn delete_template_test_by_template_id() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("DELETE", "/templates/12345")).respond_with(
                json_encoded(json!({
                    "ErrorCode": 0,
                    "Message": "OK"
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = DeleteTemplateRequest::builder()
            .id(TemplateIdOrAlias::TemplateId(12345))
            .build();

        print!("{}\n", req.endpoint());

        req.execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");
    }

    #[tokio::test]
    pub async fn delete_template_test_by_alias() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path(
                "DELETE",
                "/templates/my-template-alias",
            ))
            .respond_with(json_encoded(json!({
                "ErrorCode": 0,
                "Message": "OK"
            }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = DeleteTemplateRequest::builder()
            .id(TemplateIdOrAlias::Alias(String::from(ALIAS)))
            .build();

        print!("{}\n", req.endpoint());

        req.execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");
    }
}
