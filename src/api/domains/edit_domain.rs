use std::borrow::Cow;

use crate::api::domains::DomainDetails;
use crate::Endpoint;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Edit an existing domain. Only the ReturnPathDomain can be modified.
///
/// ```
/// use postmark::api::domains::EditDomainRequest;
/// let req = EditDomainRequest::builder()
///   .domain_id(36735)
///   .return_path_domain("pm-bounces.example.com")
///   .build();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct EditDomainRequest {
    /// Unique ID of the domain to edit.
    #[serde(skip)]
    pub domain_id: i64,
    /// Custom Return-Path domain. Must be a subdomain of your From Email domain
    /// with a CNAME record pointing to `pm.mtasv.net`.
    #[builder(setter(into))]
    pub return_path_domain: String,
}

impl Endpoint for EditDomainRequest {
    type Request = EditDomainRequest;
    type Response = DomainDetails;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/domains/{}", self.domain_id).into()
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

    const DOMAIN_ID: i64 = 36736;

    #[tokio::test]
    pub async fn edit_domain() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path(
                "PUT",
                format!("/domains/{DOMAIN_ID}"),
            ))
            .respond_with(json_encoded(json!({
                "Name": "example.com",
                "SPFVerified": false,
                "SPFHost": "example.com",
                "SPFTextValue": "v=spf1 a mx include:spf.mtasv.net ~all",
                "DKIMVerified": false,
                "WeakDKIM": false,
                "DKIMHost": "20160921046319pm._domainkey.example.com",
                "DKIMTextValue": "k=rsa;p=MIGfMA0GDRrFQJc5dZEBAQUAA4GNADCBiQKBgQCFn...",
                "DKIMPendingHost": "20131031155228pm._domainkey.example.com",
                "DKIMPendingTextValue": "k=rsa;p=MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCFn...",
                "DKIMRevokedHost": "",
                "DKIMRevokedTextValue": "",
                "SafeToRemoveRevokedKeyFromDNS": false,
                "DKIMUpdateStatus": "Pending",
                "ReturnPathDomain": "pm-bounces.example.com",
                "ReturnPathDomainVerified": false,
                "ReturnPathDomainCNAMEValue": "pm.mtasv.net",
                "ID": 36736
            }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = EditDomainRequest::builder()
            .domain_id(DOMAIN_ID)
            .return_path_domain("pm-bounces.example.com")
            .build();

        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            json!({
                "ReturnPathDomain": "pm-bounces.example.com",
            })
        );

        let resp = req
            .execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");

        assert_eq!(resp.return_path_domain, "pm-bounces.example.com");
        assert_eq!(resp.id, DOMAIN_ID);
    }
}
