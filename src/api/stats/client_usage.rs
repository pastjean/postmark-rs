use std::borrow::Cow;

use serde::Serialize;

use crate::api::stats::{build_stats_endpoint, StatsCountResponse, StatsQuery};
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ClientUsageRequest {
    #[serde(flatten)]
    pub query: StatsQuery,
}

impl Endpoint for ClientUsageRequest {
    type Request = ClientUsageRequest;
    type Response = StatsCountResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        build_stats_endpoint("/stats/outbound/emailclients", &self.query)
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
    fn client_usage_uses_stats_outbound_emailclients_get() {
        let req = ClientUsageRequest::default();

        assert_eq!(req.method(), http::Method::GET);
        assert_eq!(req.endpoint(), "/stats/outbound/emailclients");
    }
}
