use std::borrow::Cow;

use crate::Endpoint;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// Delete a domain.
///
/// ```
/// use postmark::api::domains::DeleteDomainRequest;
/// let req = DeleteDomainRequest::builder()
///   .domain_id(36735)
///   .build();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct DeleteDomainRequest {
    /// Unique ID of the domain to delete.
    #[serde(skip)]
    pub domain_id: isize,
}

/// Response for the [`DeleteDomainRequest`] endpoint.
///
/// On success, `error_code` will be 0.
/// On failure, details will be held in `error_code` and `message`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteDomainResponse {
    /// [API Error codes](https://postmarkapp.com/developer/api/overview#error-codes)
    pub error_code: i64,
    /// Associated success or error message.
    pub message: String,
}

impl Endpoint for DeleteDomainRequest {
    type Request = DeleteDomainRequest;
    type Response = DeleteDomainResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/domains/{}", self.domain_id).into()
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

    use crate::Query;
    use crate::reqwest::PostmarkClient;

    use super::*;

    const DOMAIN_ID: isize = 36735;

    #[tokio::test]
    pub async fn delete_domain() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path(
                "DELETE",
                format!("/domains/{DOMAIN_ID}"),
            ))
            .respond_with(json_encoded(json!({
                "ErrorCode": 0,
                "Message": "Domain example.com removed."
            }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = DeleteDomainRequest::builder().domain_id(DOMAIN_ID).build();

        let resp = req
            .execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");

        assert_eq!(resp.error_code, 0);
        assert_eq!(resp.message, "Domain example.com removed.");
    }
}
