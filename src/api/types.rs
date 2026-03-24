//! Legacy shared API types.
//!
//! New code should prefer IDs defined next to endpoint modules (for example
//! `api::server::ServerId`, `api::domains::DomainId`).

macro_rules! id_type {
    ($vis:vis $name:ident) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            Default,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            serde::Serialize,
            serde::Deserialize,
        )]
        #[serde(transparent)]
        #[repr(transparent)]
        $vis struct $name(i64);

        impl $name {
            pub const fn new(value: i64) -> Self {
                Self(value)
            }

            pub const fn get(self) -> i64 {
                self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<i64> for $name {
            fn from(value: i64) -> Self {
                Self(value)
            }
        }

        impl From<$name> for i64 {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl PartialEq<i64> for $name {
            fn eq(&self, other: &i64) -> bool {
                self.0 == *other
            }
        }

        impl PartialEq<$name> for i64 {
            fn eq(&self, other: &$name) -> bool {
                *self == other.0
            }
        }
    };
}

pub(crate) use id_type;

pub type ServerId = crate::api::server::ServerId;
pub type DomainId = crate::api::domains::DomainId;
pub type TemplateId = crate::api::templates::TemplateId;
pub type WebhookId = crate::api::webhooks::WebhookId;
pub type MessageNumericId = i64;
pub type ErrorCode = i64;
pub type MessageId = String;
