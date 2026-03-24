use std::borrow::Cow;

use crate::Endpoint;
use crate::api::domains::DomainDetails;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Verify Return-Path DNS record for the specified domain.
///
/// ```
/// use postmark::api::domains::VerifyReturnPathRequest;
/// let req = VerifyReturnPathRequest::builder()
///   .domain_id(36735)
///   .build();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct VerifyReturnPathRequest {
    /// Unique ID of the domain whose Return-Path DNS record should be verified.
    #[serde(skip)]
    pub domain_id: isize,
}

impl Endpoint for VerifyReturnPathRequest {
    type Request = VerifyReturnPathRequest;
    type Response = DomainDetails;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/domains/{}/verifyReturnPath", self.domain_id).into()
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
    use httptest::{Expectation, Server, responders::*};
    use serde_json::json;

    use crate::Query;
    use crate::reqwest::PostmarkClient;

    use super::*;

    const DOMAIN_ID: isize = 36735;

    #[tokio::test]
    pub async fn verify_return_path() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path(
                "PUT",
                format!("/domains/{DOMAIN_ID}/verifyReturnPath"),
            ))
            .respond_with(json_encoded(json!({
                "Name": "postmarkapp.com",
                "SPFVerified": true,
                "SPFHost": "postmarkapp.com",
                "SPFTextValue": "v=spf1 a mx include:spf.mtasv.net ~all",
                "DKIMVerified": false,
                "WeakDKIM": false,
                "DKIMHost": "jan2013pm._domainkey.postmarkapp.com",
                "DKIMTextValue": "k=rsa;p=MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDJ...",
                "DKIMPendingHost": "20131031155228pm._domainkey.postmarkapp.com",
                "DKIMPendingTextValue": "k=rsa;p=MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCFn...",
                "DKIMRevokedHost": "",
                "DKIMRevokedTextValue": "",
                "SafeToRemoveRevokedKeyFromDNS": false,
                "DKIMUpdateStatus": "Pending",
                "ReturnPathDomain": "pm-bounces.postmarkapp.com",
                "ReturnPathDomainVerified": false,
                "ReturnPathDomainCNAMEValue": "pm.mtasv.net",
                "ID": 36735
            }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = VerifyReturnPathRequest::builder()
            .domain_id(DOMAIN_ID)
            .build();

        let resp = req
            .execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");

        assert_eq!(resp.name, "postmarkapp.com");
        assert_eq!(resp.id, DOMAIN_ID);
    }
}
