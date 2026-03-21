# Stats API examples

```rust
use postmark::api::stats::{GetOutboundOverviewRequest, StatsQuery};
use postmark::reqwest::PostmarkClient;
use postmark::Query;

async fn get_overview() {
    let client = PostmarkClient::builder()
        .server_token("<server-token>")
        .build();

    let req = GetOutboundOverviewRequest::builder()
        .query(StatsQuery {
            fromdate: Some("2026-01-01".to_string()),
            todate: Some("2026-01-31".to_string()),
            messagestream: Some("outbound".to_string()),
            ..Default::default()
        })
        .build();

    let overview = req.execute(&client).await.unwrap();
    assert!(overview.sent >= 0);
}
```
