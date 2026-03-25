use crate::{Endpoint, api::endpoint_with_path_segment};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use typed_builder::TypedBuilder;

use super::*;

/// Delete an existing e-mail template. The template may be specified by its template id or alias
/// using the [`TemplateIdOrAlias`] enumeration.
///
/// ```
/// use postmark::api::{Body, templates::{DeleteTemplateRequest, TemplateIdOrAlias}};
/// let req = DeleteTemplateRequest::builder()
///   .id(TemplateIdOrAlias::TemplateId(12345.into()))
///   .build();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct DeleteTemplateRequest {
    /// ID of template or template alias
    #[builder(setter(into))]
    pub id: TemplateIdOrAlias,
}

/// Response for the [`DeleteTemplateRequest`] Endpoint.
///
/// On a success, `error_code` will be 0 and message "OK".
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
        endpoint_with_path_segment("/templates", &self.id.to_string())
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
    use httptest::{Expectation, Server, responders::*};
    use serde_json::json;

    use super::*;
    use crate::Query;
    use crate::reqwest::PostmarkClient;

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
            .id(TemplateIdOrAlias::TemplateId(12345.into()))
            .build();

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

        req.execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");
    }

    #[tokio::test]
    pub async fn delete_template_test_should_not_error_on_postmark_error() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path(
                "DELETE", 
                "/templates/12345"
            )).respond_with(
                json_encoded(json!({
                    "ErrorCode": 1101,
                    "Message": "The TemplateId,  LayoutTemplate, or Alias references a Template that does not exist, or is not associated with the Server specified for this request."                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = DeleteTemplateRequest::builder()
            .id(TemplateIdOrAlias::TemplateId(12345.into()))
            .build();

        req.execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");
    }
}
