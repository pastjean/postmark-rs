//! Domains API endpoints for managing domain details in a Postmark account.

pub use create_domain::*;
pub use delete_domain::*;
pub use edit_domain::*;
pub use get_domain::*;
pub use list_domains::*;
pub use rotate_dkim::*;
pub use verify_dkim::*;
pub use verify_return_path::*;
pub use verify_spf::*;

use serde::{Deserialize, Serialize};

mod create_domain;
mod delete_domain;
mod edit_domain;
mod get_domain;
mod list_domains;
mod rotate_dkim;
mod verify_dkim;
mod verify_return_path;
mod verify_spf;

/// Status of a DKIM update operation.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum DkimUpdateStatus {
    /// DKIM renewal or setup is in progress.
    #[default]
    Pending,
    /// All DNS TXT records are up to date and any pending operations are finished.
    Verified,
    /// DKIM verification failed.
    Failed,
    /// Catch-all for any status not yet represented in this enum.
    #[serde(untagged)]
    Unknown(String),
}

/// Summary of a domain as returned by the list domains endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DomainSummary {
    /// Domain name.
    pub name: String,
    /// Deprecated. See Postmark's blog post on why SPF records are no longer required.
    #[serde(rename = "SPFVerified")]
    pub spf_verified: bool,
    /// Whether DKIM has ever been verified for the domain.
    /// Once verified, this stays true even if the record is later removed from DNS.
    #[serde(rename = "DKIMVerified")]
    pub dkim_verified: bool,
    /// Whether DKIM is using a strength weaker than 1024 bit.
    #[serde(rename = "WeakDKIM")]
    pub weak_dkim: bool,
    /// Whether the Return-Path domain is actively being used.
    pub return_path_domain_verified: bool,
    /// Unique ID of the domain.
    #[serde(rename = "ID")]
    pub id: isize,
}

/// Full domain details as returned by get, create, edit, and verify endpoints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DomainDetails {
    /// Domain name.
    pub name: String,
    /// Deprecated. See Postmark's blog post on why SPF records are no longer required.
    #[serde(rename = "SPFVerified")]
    pub spf_verified: bool,
    /// Host name used for the SPF configuration.
    #[serde(rename = "SPFHost")]
    pub spf_host: String,
    /// Value that can be optionally set up with your DNS host for SPF verification.
    #[serde(rename = "SPFTextValue")]
    pub spf_text_value: String,
    /// Whether DKIM has ever been verified for the domain.
    /// Once verified, this stays true even if the record is later removed from DNS.
    #[serde(rename = "DKIMVerified")]
    pub dkim_verified: bool,
    /// Whether DKIM is using a strength weaker than 1024 bit.
    /// If so, you can request a new DKIM via [`RotateDkimRequest`].
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
    /// Custom Return-Path domain. Must be a subdomain of your From Email domain
    /// with a CNAME record pointing to `pm.mtasv.net`.
    pub return_path_domain: String,
    /// Whether the Return-Path domain is actively being used.
    pub return_path_domain_verified: bool,
    /// The CNAME DNS record that Postmark expects to find at the Return-Path domain.
    #[serde(rename = "ReturnPathDomainCNAMEValue")]
    pub return_path_domain_cname_value: String,
    /// Unique ID of the domain.
    #[serde(rename = "ID")]
    pub id: isize,
}
