# Sender signatures examples

```rust
use postmark::api::signatures::{CreateSignatureRequest, ListSignaturesRequest};
use postmark::reqwest::PostmarkClient;
use postmark::Query;

async fn signatures_examples() {
    let client = PostmarkClient::builder()
        .account_token("<account-token>")
        .build();

    let create_req = CreateSignatureRequest::builder()
        .from_email("john@example.com".to_string())
        .name("John".to_string())
        .build();

    let created = create_req.execute(&client).await.unwrap();

    let list_req = ListSignaturesRequest::builder().count(50).offset(0).build();
    let list = list_req.execute(&client).await.unwrap();

    assert!(list.sender_signatures.iter().any(|s| s.id == created.id));
}
```
