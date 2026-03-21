use std::borrow::Cow;

use crate::api::domains::DomainDetails;
use crate::Endpoint;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Create a new domain.
///
/// ```
/// use postmark::api::domains::CreateDomainRequest;
/// let req = CreateDomainRequest::builder()
///   .name("example.com")
///   .return_path_domain("pm-bounces.example.com")
///   .build();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct CreateDomainRequest {
    /// Domain name to register.
    #[builder(setter(into))]
    pub name: String,
    /// Optional custom Return-Path domain. Must be a subdomain of your From Email domain
    /// with a CNAME record pointing to `pm.mtasv.net`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub return_path_domain: Option<String>,
}

impl Endpoint for CreateDomainRequest {
    type Request = CreateDomainRequest;
    type Response = DomainDetails;

    fn endpoint(&self) -> Cow<'static, str> {
        "/domains".into()
    }

    fn body(&self) -> &Self::Request {
        self
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
    pub async fn create_domain() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("POST", "/domains")).respond_with(
                json_encoded(json!({
                    "Name": "newdomain.com",
                    "SPFVerified": false,
                    "SPFHost": "newdomain.com",
                    "SPFTextValue": "v=spf1 a mx include:spf.mtasv.net ~all",
                    "DKIMVerified": false,
                    "WeakDKIM": false,
                    "DKIMHost": "",
                    "DKIMTextValue": "",
                    "DKIMPendingHost": "20131031155228pm._domainkey.newdomain.com",
                    "DKIMPendingTextValue": "k=rsa;p=MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCFn...",
                    "DKIMRevokedHost": "",
                    "DKIMRevokedTextValue": "",
                    "SafeToRemoveRevokedKeyFromDNS": false,
                    "DKIMUpdateStatus": "Pending",
                    "ReturnPathDomain": "pm-bounces.newdomain.com",
                    "ReturnPathDomainVerified": false,
                    "ReturnPathDomainCNAMEValue": "pm.mtasv.net",
                    "ID": 36736
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = CreateDomainRequest::builder()
            .name("newdomain.com")
            .return_path_domain("pm-bounces.newdomain.com")
            .build();

        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            json!({
                "Name": "newdomain.com",
                "ReturnPathDomain": "pm-bounces.newdomain.com",
            })
        );

        let resp = req
            .execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");

        assert_eq!(resp.name, "newdomain.com");
        assert_eq!(resp.id, 36736);
    }

    #[tokio::test]
    pub async fn create_domain_without_return_path() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("POST", "/domains")).respond_with(
                json_encoded(json!({
                    "Name": "newdomain.com",
                    "SPFVerified": false,
                    "SPFHost": "newdomain.com",
                    "SPFTextValue": "v=spf1 a mx include:spf.mtasv.net ~all",
                    "DKIMVerified": false,
                    "WeakDKIM": false,
                    "DKIMHost": "",
                    "DKIMTextValue": "",
                    "DKIMPendingHost": "",
                    "DKIMPendingTextValue": "",
                    "DKIMRevokedHost": "",
                    "DKIMRevokedTextValue": "",
                    "SafeToRemoveRevokedKeyFromDNS": false,
                    "DKIMUpdateStatus": "Pending",
                    "ReturnPathDomain": "",
                    "ReturnPathDomainVerified": false,
                    "ReturnPathDomainCNAMEValue": "",
                    "ID": 36737
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = CreateDomainRequest::builder()
            .name("newdomain.com")
            .build();

        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            json!({
                "Name": "newdomain.com",
            })
        );

        req.execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");
    }
}
