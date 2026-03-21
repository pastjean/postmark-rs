use std::borrow::Cow;

use crate::api::stats::{stats_endpoint, StatsQuery};
use crate::Endpoint;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct GetOutboundOverviewRequest {
    #[builder(default)]
    #[serde(skip)]
    pub query: StatsQuery,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OutboundOverviewResponse {
    pub sent: isize,
    pub bounced: isize,
    #[serde(rename = "SMTPApiErrors")]
    pub smtp_api_errors: isize,
    pub bounce_rate: f64,
    pub spam_complaints: isize,
    pub spam_complaints_rate: f64,
    pub opens: isize,
    pub unique_opens: isize,
    pub total_clicks: isize,
    pub unique_links_clicked: isize,
    pub total_tracked_links_sent: isize,
    pub tracked: isize,
    pub with_link_tracking: isize,
    pub with_open_tracking: isize,
    pub with_client_recorded: isize,
    pub with_platform_recorded: isize,
}

impl Endpoint for GetOutboundOverviewRequest {
    type Request = GetOutboundOverviewRequest;
    type Response = OutboundOverviewResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        stats_endpoint("/stats/outbound", &self.query)
    }

    fn body(&self) -> &Self::Request {
        self
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
}

macro_rules! stats_req {
    ($name:ident, $path:expr, $resp:ty) => {
        #[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
        #[serde(rename_all = "PascalCase")]
        pub struct $name {
            #[builder(default)]
            #[serde(skip)]
            pub query: StatsQuery,
        }

        impl Endpoint for $name {
            type Request = $name;
            type Response = $resp;

            fn endpoint(&self) -> Cow<'static, str> {
                stats_endpoint($path, &self.query)
            }

            fn body(&self) -> &Self::Request {
                self
            }

            fn method(&self) -> http::Method {
                http::Method::GET
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SentCountsResponse {
    pub days: Vec<Value>,
    pub sent: isize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BounceCountsResponse {
    pub days: Vec<Value>,
    pub hard_bounce: isize,
    #[serde(rename = "SMTPApiError")]
    pub smtp_api_error: isize,
    pub soft_bounce: isize,
    pub transient: isize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SpamCountsResponse {
    pub days: Vec<Value>,
    pub spam_complaint: isize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TrackedCountsResponse {
    pub days: Vec<Value>,
    pub tracked: isize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OpensCountsResponse {
    pub days: Vec<Value>,
    pub opens: isize,
    pub unique: isize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OpensPlatformsResponse {
    pub days: Vec<Value>,
    pub desktop: isize,
    pub mobile: isize,
    pub unknown: isize,
    #[serde(rename = "WebMail")]
    pub web_mail: isize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamicKeyCountsResponse {
    #[serde(rename = "Days")]
    pub days: Vec<Value>,
    #[serde(flatten)]
    pub totals: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClickCountsResponse {
    pub days: Vec<Value>,
    pub clicks: isize,
    pub unique: isize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClickPlatformsResponse {
    pub days: Vec<Value>,
    pub desktop: isize,
    pub mobile: isize,
    pub unknown: isize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClickLocationResponse {
    pub days: Vec<Value>,
    #[serde(rename = "HTML")]
    pub html: isize,
    #[serde(rename = "Text")]
    pub text: isize,
}

stats_req!(
    GetSentCountsRequest,
    "/stats/outbound/sends",
    SentCountsResponse
);
stats_req!(
    GetBounceCountsRequest,
    "/stats/outbound/bounces",
    BounceCountsResponse
);
stats_req!(
    GetSpamComplaintsRequest,
    "/stats/outbound/spam",
    SpamCountsResponse
);
stats_req!(
    GetTrackedEmailCountsRequest,
    "/stats/outbound/tracked",
    TrackedCountsResponse
);
stats_req!(
    GetEmailOpenCountsRequest,
    "/stats/outbound/opens",
    OpensCountsResponse
);
stats_req!(
    GetEmailPlatformUsageRequest,
    "/stats/outbound/opens/platforms",
    OpensPlatformsResponse
);
stats_req!(
    GetEmailClientUsageRequest,
    "/stats/outbound/opens/emailclients",
    DynamicKeyCountsResponse
);
stats_req!(
    GetClickCountsRequest,
    "/stats/outbound/clicks",
    ClickCountsResponse
);
stats_req!(
    GetBrowserUsageRequest,
    "/stats/outbound/clicks/browserfamilies",
    DynamicKeyCountsResponse
);
stats_req!(
    GetBrowserPlatformUsageRequest,
    "/stats/outbound/clicks/platforms",
    ClickPlatformsResponse
);
stats_req!(
    GetClickLocationRequest,
    "/stats/outbound/clicks/location",
    ClickLocationResponse
);

#[cfg(test)]
mod tests {
    use httptest::matchers::request;
    use httptest::{responders::*, Expectation, Server};
    use serde_json::json;

    use crate::reqwest::PostmarkClient;
    use crate::Query;

    use super::*;

    #[tokio::test]
    async fn get_outbound_overview() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/stats/outbound")).respond_with(
                json_encoded(json!({
                    "Sent":615,"Bounced":64,"SMTPApiErrors":25,"BounceRate":10.406,
                    "SpamComplaints":10,"SpamComplaintsRate":1.626,
                    "Opens":166,"UniqueOpens":26,"TotalClicks":72,
                    "UniqueLinksClicked":30,"TotalTrackedLinksSent":60,
                    "Tracked":111,"WithLinkTracking":90,"WithOpenTracking":51,
                    "WithClientRecorded":14,"WithPlatformRecorded":10
                })),
            ),
        );
        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = GetOutboundOverviewRequest::builder().build();
        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.sent, 615);
    }

    #[tokio::test]
    async fn get_sent_counts() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/stats/outbound/sends"))
                .respond_with(json_encoded(json!({"Days": [], "Sent": 615}))),
        );
        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = GetSentCountsRequest::builder().build();
        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.sent, 615);
    }

    #[tokio::test]
    async fn get_bounce_counts() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/stats/outbound/bounces"))
                .respond_with(json_encoded(json!({"Days":[],"HardBounce":12,"SMTPApiError":25,"SoftBounce":36,"Transient":16}))),
        );
        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = GetBounceCountsRequest::builder().build();
        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.hard_bounce, 12);
    }

    #[tokio::test]
    async fn get_spam_counts() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/stats/outbound/spam"))
                .respond_with(json_encoded(json!({"Days":[],"SpamComplaint":10}))),
        );
        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = GetSpamComplaintsRequest::builder().build();
        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.spam_complaint, 10);
    }

    #[tokio::test]
    async fn get_tracked_counts() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/stats/outbound/tracked"))
                .respond_with(json_encoded(json!({"Days":[],"Tracked":111}))),
        );
        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = GetTrackedEmailCountsRequest::builder().build();
        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.tracked, 111);
    }

    #[tokio::test]
    async fn get_open_counts() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/stats/outbound/opens"))
                .respond_with(json_encoded(json!({"Days":[],"Opens":166,"Unique":26}))),
        );
        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = GetEmailOpenCountsRequest::builder().build();
        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.unique, 26);
    }

    #[tokio::test]
    async fn get_open_platforms() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path(
                "GET",
                "/stats/outbound/opens/platforms",
            ))
            .respond_with(json_encoded(
                json!({"Days":[],"Desktop":4,"Mobile":2,"Unknown":2,"WebMail":2}),
            )),
        );
        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = GetEmailPlatformUsageRequest::builder().build();
        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.desktop, 4);
    }

    #[tokio::test]
    async fn get_open_clients() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path(
                "GET",
                "/stats/outbound/opens/emailclients",
            ))
            .respond_with(json_encoded(json!({"Days":[],"Apple Mail":6}))),
        );
        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = GetEmailClientUsageRequest::builder().build();
        let resp = req.execute(&client).await.expect("json decode");
        assert!(resp.totals.get("Apple Mail").is_some());
    }

    #[tokio::test]
    async fn get_click_counts() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/stats/outbound/clicks"))
                .respond_with(json_encoded(json!({"Days":[],"Clicks":166,"Unique":26}))),
        );
        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = GetClickCountsRequest::builder().build();
        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.clicks, 166);
    }

    #[tokio::test]
    async fn get_browser_usage() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path(
                "GET",
                "/stats/outbound/clicks/browserfamilies",
            ))
            .respond_with(json_encoded(json!({"Days":[],"Google Chrome":6}))),
        );
        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = GetBrowserUsageRequest::builder().build();
        let resp = req.execute(&client).await.expect("json decode");
        assert!(resp.totals.get("Google Chrome").is_some());
    }

    #[tokio::test]
    async fn get_browser_platform_usage() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path(
                "GET",
                "/stats/outbound/clicks/platforms",
            ))
            .respond_with(json_encoded(
                json!({"Days":[],"Desktop":4,"Mobile":2,"Unknown":2}),
            )),
        );
        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = GetBrowserPlatformUsageRequest::builder().build();
        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.mobile, 2);
    }

    #[tokio::test]
    async fn get_click_location() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path(
                "GET",
                "/stats/outbound/clicks/location",
            ))
            .respond_with(json_encoded(json!({"Days":[],"HTML":4,"Text":4}))),
        );
        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = GetClickLocationRequest::builder().build();
        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.html, 4);
    }
}
