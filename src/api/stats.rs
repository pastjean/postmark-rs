//! You'll find in stats API related endpoints.

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use typed_builder::TypedBuilder;
use url::form_urlencoded::Serializer;

mod bounce_counts;
mod browser_platform_usage;
mod browser_usage;
mod click_counts;
mod click_location;
mod client_usage;
mod open_counts;
mod outbound_overview;
mod platform_usage;
mod sent_counts;
mod spam_counts;
mod tracked_email_counts;

pub use bounce_counts::*;
pub use browser_platform_usage::*;
pub use browser_usage::*;
pub use click_counts::*;
pub use click_location::*;
pub use client_usage::*;
pub use open_counts::*;
pub use outbound_overview::*;
pub use platform_usage::*;
pub use sent_counts::*;
pub use spam_counts::*;
pub use tracked_email_counts::*;

#[derive(Debug, Clone, PartialEq, Serialize, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
#[serde(rename_all = "PascalCase")]
pub struct StatsQuery {
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub tag: Option<String>,
    pub message_stream: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OutboundOverviewResponse {
    pub sent: i64,
    pub bounced: i64,
    pub spam_complaints: i64,
    pub tracked: i64,
    pub opens: i64,
    pub clicks: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StatsCountResponse {
    pub sent: Option<i64>,
    pub bounced: Option<i64>,
    pub spam_complaints: Option<i64>,
    pub tracked: Option<i64>,
    pub opens: Option<i64>,
    pub clicks: Option<i64>,
    pub days: Option<Vec<StatsBucket>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StatsBucket {
    pub name: String,
    pub count: i64,
    pub percent: Option<f64>,
}

pub(crate) fn build_stats_endpoint(path: &str, query: &StatsQuery) -> Cow<'static, str> {
    let mut serializer = Serializer::new(String::new());

    if let Some(from_date) = query.from_date.as_deref() {
        serializer.append_pair("fromdate", from_date);
    }
    if let Some(to_date) = query.to_date.as_deref() {
        serializer.append_pair("todate", to_date);
    }
    if let Some(tag) = query.tag.as_deref() {
        serializer.append_pair("tag", tag);
    }
    if let Some(message_stream) = query.message_stream.as_deref() {
        serializer.append_pair("messagestream", message_stream);
    }

    let query = serializer.finish();
    if query.is_empty() {
        path.to_string().into()
    } else {
        format!("{path}?{query}").into()
    }
}
