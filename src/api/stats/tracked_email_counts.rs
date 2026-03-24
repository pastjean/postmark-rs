use std::borrow::Cow;

use serde::Serialize;

use crate::api::stats::{build_stats_endpoint, StatsCountResponse, StatsQuery};
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct TrackedEmailCountsRequest {
    #[serde(flatten)]
    pub query: StatsQuery,
}

impl Endpoint for TrackedEmailCountsRequest {
    type Request = TrackedEmailCountsRequest;
    type Response = StatsCountResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        build_stats_endpoint("/stats/outbound/tracked", &self.query)
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
    fn tracked_counts_uses_stats_outbound_tracked_get() {
        let req = TrackedEmailCountsRequest::default();

        assert_eq!(req.method(), http::Method::GET);
        assert_eq!(req.endpoint(), "/stats/outbound/tracked");
    }
}
