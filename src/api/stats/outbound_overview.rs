use std::borrow::Cow;

use serde::Serialize;

use crate::api::stats::{build_stats_endpoint, OutboundOverviewResponse, StatsQuery};
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OutboundOverviewRequest {
    #[serde(flatten)]
    pub query: StatsQuery,
}

impl Endpoint for OutboundOverviewRequest {
    type Request = OutboundOverviewRequest;
    type Response = OutboundOverviewResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        build_stats_endpoint("/stats/outbound", &self.query)
    }

    fn body(&self) -> &Self::Request {
        self
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outbound_overview_uses_stats_outbound_get() {
        let req = OutboundOverviewRequest::default();

        assert_eq!(req.method(), http::Method::GET);
        assert_eq!(req.endpoint(), "/stats/outbound");
    }
}
