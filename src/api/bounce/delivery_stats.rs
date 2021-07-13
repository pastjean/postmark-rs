use crate::Endpoint;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct DeliveryStatsRequest {}

impl Endpoint for DeliveryStatsRequest {
    type Request = ();
    type Response = DeliveryStatsResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "/deliverystats".into()
    }

    fn body(&self) -> &Self::Request {
        &()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]

pub struct DeliveryStatsResponse {
    #[serde(rename = "InactiveMails")]
    /// Number of inactive emails
    pub inactive_mails: i64,
    /// List of [bounce types](https://postmarkapp.com/developer/api/bounce-api#bounce-types) with total counts.
    pub bounces: Vec<Bounce>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Bounce {
    pub name: String,
    pub count: i64,
    pub type_field: Option<String>,
}
