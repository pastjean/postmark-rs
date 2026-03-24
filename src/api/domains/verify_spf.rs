use std::borrow::Cow;

use crate::Endpoint;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// Verify SPF record for the specified domain.
///
/// ```
/// use postmark::api::domains::VerifySpfRequest;
/// let req = VerifySpfRequest::builder()
///   .domain_id(36735)
///   .build();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct VerifySpfRequest {
    /// Unique ID of the domain whose SPF record should be verified.
    #[serde(skip)]
    pub domain_id: isize,
}

/// Response for the [`VerifySpfRequest`] endpoint.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VerifySpfResponse {
    /// Host name used for the SPF configuration.
    #[serde(rename = "SPFHost")]
    pub spf_host: String,
    /// Whether SPF DNS text record has been setup correctly.
    #[serde(rename = "SPFVerified")]
    pub spf_verified: bool,
    /// Value that can be optionally set up with your DNS host for SPF verification.
    #[serde(rename = "SPFTextValue")]
    pub spf_text_value: String,
}

impl Endpoint for VerifySpfRequest {
    type Request = VerifySpfRequest;
    type Response = VerifySpfResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/domains/{}/verifyspf", self.domain_id).into()
    }

    fn body(&self) -> &Self::Request {
        self
    }

    fn method(&self) -> http::Method {
        http::Method::POST
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
    pub async fn verify_spf() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path(
                "POST",
                format!("/domains/{DOMAIN_ID}/verifyspf"),
            ))
            .respond_with(json_encoded(json!({
                "SPFHost": "postmarkapp.com",
                "SPFVerified": true,
                "SPFTextValue": "v=spf1 a mx include:spf.mtasv.net ~all",
            }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = VerifySpfRequest::builder().domain_id(DOMAIN_ID).build();

        let resp = req
            .execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");

        assert!(resp.spf_verified);
        assert_eq!(resp.spf_host, "postmarkapp.com");
    }
}
