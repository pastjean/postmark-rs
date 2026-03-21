use std::borrow::Cow;

use crate::api::domains::DomainDetails;
use crate::Endpoint;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Verify DKIM keys for the specified domain.
///
/// ```
/// use postmark::api::domains::VerifyDkimRequest;
/// let req = VerifyDkimRequest::builder()
///   .domain_id(36735)
///   .build();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct VerifyDkimRequest {
    /// Unique ID of the domain whose DKIM keys should be verified.
    #[serde(skip)]
    pub domain_id: isize,
}

impl Endpoint for VerifyDkimRequest {
    type Request = VerifyDkimRequest;
    type Response = DomainDetails;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/domains/{}/verifyDkim", self.domain_id).into()
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

    const DOMAIN_ID: isize = 36735;

    #[tokio::test]
    pub async fn verify_dkim() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path(
                "PUT",
                format!("/domains/{DOMAIN_ID}/verifyDkim"),
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

        let req = VerifyDkimRequest::builder().domain_id(DOMAIN_ID).build();

        let resp = req
            .execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");

        assert_eq!(resp.name, "postmarkapp.com");
        assert_eq!(resp.id, DOMAIN_ID);
    }
}
