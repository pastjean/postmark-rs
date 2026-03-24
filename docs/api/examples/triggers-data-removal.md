# Triggers and Data Removal examples

```rust
use postmark::api::data_removal::{CreateDataRemovalRequest, GetDataRemovalStatusRequest};
use postmark::api::triggers::{
    CreateInboundRuleTriggerRequest, DeleteInboundRuleTriggerRequest, ListInboundRuleTriggersRequest,
};
use postmark::reqwest::PostmarkClient;
use postmark::Query;

async fn trigger_examples() {
    let server_client = PostmarkClient::builder()
        .server_token("<server-token>")
        .build();

    let create = CreateInboundRuleTriggerRequest::builder()
        .rule("blocked@example.com".to_string())
        .build()
        .execute(&server_client)
        .await
        .unwrap();

    let _rules = ListInboundRuleTriggersRequest::builder()
        .count(50)
        .offset(0)
        .build()
        .execute(&server_client)
        .await
        .unwrap();

    let _ = DeleteInboundRuleTriggerRequest::builder()
        .trigger_id(create.id)
        .build()
        .execute(&server_client)
        .await
        .unwrap();
}

async fn data_removal_examples() {
    let account_client = PostmarkClient::builder()
        .account_token("<account-token>")
        .build();

    let req = CreateDataRemovalRequest::builder()
        .requested_by("owner@example.com".to_string())
        .requested_for("recipient@example.com".to_string())
        .notify_when_completed(true)
        .build();

    let created = req.execute(&account_client).await.unwrap();

    let _status = GetDataRemovalStatusRequest::builder()
        .data_removal_id(created.id)
        .build()
        .execute(&account_client)
        .await
        .unwrap();
}
```
