# Bulk API examples

```rust
use postmark::api::bulk::{BulkMessage, GetBulkStatusRequest, SendBulkEmailRequest};
use postmark::reqwest::PostmarkClient;
use postmark::Query;

async fn send_and_check_bulk() {
    let client = PostmarkClient::builder()
        .server_token("<server-token>")
        .build();

    let send_req = SendBulkEmailRequest::builder()
        .from("sender@example.com".to_string())
        .subject("Hello {{FirstName}}")
        .text_body("Hi {{FirstName}}")
        .message_stream("broadcast")
        .messages(vec![BulkMessage {
            to: "receiver@example.com".to_string(),
            ..Default::default()
        }])
        .build();

    let accepted = send_req.execute(&client).await.unwrap();

    let status_req = GetBulkStatusRequest::builder()
        .bulk_request_id(accepted.id)
        .build();

    let status = status_req.execute(&client).await.unwrap();
    assert!(status.percentage_completed >= 0.0);
}
```
