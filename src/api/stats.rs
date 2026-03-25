//! Stats API endpoints.

use std::borrow::Cow;

use crate::api::endpoint_with_query;
use serde::Serialize;
use url::form_urlencoded::Serializer;

mod stats_outbound;
pub use stats_outbound::*;

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct StatsQuery {
    pub tag: Option<String>,
    pub fromdate: Option<String>,
    pub todate: Option<String>,
    pub message_stream: Option<String>,
}

pub(crate) fn stats_endpoint(path: &str, query: &StatsQuery) -> Cow<'static, str> {
    let mut serializer = Serializer::new(String::new());
    if let Some(tag) = query.tag.as_deref() {
        serializer.append_pair("tag", tag);
    }
    if let Some(fromdate) = query.fromdate.as_deref() {
        serializer.append_pair("fromdate", fromdate);
    }
    if let Some(todate) = query.todate.as_deref() {
        serializer.append_pair("todate", todate);
    }
    if let Some(message_stream) = query.message_stream.as_deref() {
        serializer.append_pair("MessageStream", message_stream);
    }

    endpoint_with_query(path, serializer.finish())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stats_endpoint_uses_message_stream_pascal_case_param() {
        let query = StatsQuery {
            message_stream: Some("outbound".to_string()),
            ..StatsQuery::default()
        };

        let endpoint = stats_endpoint("/stats/outbound", &query);
        assert_eq!(endpoint.as_ref(), "/stats/outbound?MessageStream=outbound");
    }
}
