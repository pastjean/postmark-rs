use std::borrow::Cow;

use crate::Endpoint;
use crate::api::domains::DkimUpdateStatus;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// Request rotation of DKIM keys for the specified domain.
///
/// ```
/// use postmark::api::domains::RotateDkimRequest;
/// let req = RotateDkimRequest::builder()
///   .domain_id(36735)
///   .build();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct RotateDkimRequest {
    /// Unique ID of the domain whose DKIM keys should be rotated.
    #[serde(skip)]
    pub domain_id: isize,
}

/// Response for the [`RotateDkimRequest`] endpoint.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RotateDkimResponse {
    /// Domain name.
    pub name: String,
    /// Whether DKIM has ever been verified for the domain.
    #[serde(rename = "DKIMVerified")]
    pub dkim_verified: bool,
    /// Whether DKIM is using a strength weaker than 1024 bit.
    #[serde(rename = "WeakDKIM")]
    pub weak_dkim: bool,
    /// DNS TXT host being used to validate messages sent in.
    #[serde(rename = "DKIMHost")]
    pub dkim_host: String,
    /// DNS TXT value being used to validate messages sent in.
    #[serde(rename = "DKIMTextValue")]
    pub dkim_text_value: String,
    /// Pending DKIM DNS TXT host awaiting setup and confirmation at your registrar or DNS host.
    #[serde(rename = "DKIMPendingHost")]
    pub dkim_pending_host: String,
    /// Pending DKIM DNS TXT value awaiting confirmation at your registrar or DNS host.
    #[serde(rename = "DKIMPendingTextValue")]
    pub dkim_pending_text_value: String,
    /// The old DKIM host that Postmark has revoked after a new DKIM was confirmed.
    #[serde(rename = "DKIMRevokedHost")]
    pub dkim_revoked_host: String,
    /// The old DKIM DNS TXT value that will soon be removed from the Postmark system.
    #[serde(rename = "DKIMRevokedTextValue")]
    pub dkim_revoked_text_value: String,
    /// Whether you may safely delete the old revoked DKIM DNS TXT records.
    #[serde(rename = "SafeToRemoveRevokedKeyFromDNS")]
    pub safe_to_remove_revoked_key_from_dns: bool,
    /// DKIM update status.
    #[serde(rename = "DKIMUpdateStatus")]
    pub dkim_update_status: DkimUpdateStatus,
    /// Unique ID of the domain.
    #[serde(rename = "ID")]
    pub id: isize,
}

impl Endpoint for RotateDkimRequest {
    type Request = RotateDkimRequest;
    type Response = RotateDkimResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/domains/{}/rotatedkim", self.domain_id).into()
    }

    fn body(&self) -> &Self::Request {
        self
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
    pub async fn rotate_dkim() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path(
                "POST",
                format!("/domains/{DOMAIN_ID}/rotatedkim"),
            ))
            .respond_with(json_encoded(json!({
                "Name": "postmarkapp.com",
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
                "ID": 36735
            }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = RotateDkimRequest::builder().domain_id(DOMAIN_ID).build();

        let resp = req
            .execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");

        assert_eq!(resp.name, "postmarkapp.com");
        assert_eq!(resp.dkim_update_status, DkimUpdateStatus::Pending);
        assert_eq!(resp.id, DOMAIN_ID);
    }
}
