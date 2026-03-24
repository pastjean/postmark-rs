//! Stats API endpoints.

use std::borrow::Cow;

use serde::Serialize;

mod stats_outbound;
pub use stats_outbound::*;

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct StatsQuery {
    pub tag: Option<String>,
    pub fromdate: Option<String>,
    pub todate: Option<String>,
    pub messagestream: Option<String>,
}

pub(crate) fn stats_endpoint(path: &str, query: &StatsQuery) -> Cow<'static, str> {
    let mut params: Vec<String> = Vec::new();
    if let Some(tag) = &query.tag {
        params.push(format!("tag={tag}"));
    }
    if let Some(fromdate) = &query.fromdate {
        params.push(format!("fromdate={fromdate}"));
    }
    if let Some(todate) = &query.todate {
        params.push(format!("todate={todate}"));
    }
    if let Some(messagestream) = &query.messagestream {
        params.push(format!("messagestream={messagestream}"));
    }

    if params.is_empty() {
        path.to_string().into()
    } else {
        format!("{path}?{}", params.join("&")).into()
    }
}
