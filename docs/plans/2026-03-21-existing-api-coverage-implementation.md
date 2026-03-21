# Existing API Coverage Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement all missing Postmark endpoints in already-present API sections.

**Architecture:** Keep endpoint-per-file modules under each existing section, with explicit request/response structs and `Endpoint` impls. Add section-local enums/models only where docs require constrained values. Drive all changes via TDD red-green-refactor with request path + decode tests.

**Tech Stack:** Rust, serde, typed-builder, httptest, tokio, reqwest, cargo test/clippy/fmt.

---

### Task 1: Templates - List Templates

**Files:**
- Create: `src/api/templates/list_templates.rs`
- Modify: `src/api/templates.rs`
- Test: `src/api/templates/list_templates.rs`

**Step 1: Write the failing test**

```rust
#[tokio::test]
async fn list_templates() {
    // Expect GET /templates?count=100&offset=0
    // Return TotalCount + Templates array
    // Assert decoded total_count and first template alias
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test api::templates::list_templates::tests::list_templates`
Expected: FAIL with unresolved module/type.

**Step 3: Write minimal implementation**

```rust
#[derive(TypedBuilder, Serialize)]
pub struct ListTemplatesRequest { pub count: isize, pub offset: isize, ... }
impl Endpoint for ListTemplatesRequest { fn endpoint(&self) -> Cow<'static, str> { ... } fn method(&self) -> http::Method { http::Method::GET } }
```

**Step 4: Run test to verify it passes**

Run: `cargo test api::templates::list_templates::tests::list_templates`
Expected: PASS.

**Step 5: Commit**

```bash
git add src/api/templates/list_templates.rs src/api/templates.rs
git commit -m "feat add templates list endpoint"
```

### Task 2: Templates - Validate Template

**Files:**
- Create: `src/api/templates/validate_template.rs`
- Modify: `src/api/templates.rs`
- Test: `src/api/templates/validate_template.rs`

**Step 1: Write the failing test**

```rust
#[tokio::test]
async fn validate_template() {
    // Expect POST /templates/validate
    // Return AllContentIsValid + Subject/HtmlBody/TextBody validation blocks
    // Assert decode for all_content_is_valid and subject.content_is_valid
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test api::templates::validate_template::tests::validate_template`
Expected: FAIL with unresolved module/type.

**Step 3: Write minimal implementation**

```rust
#[derive(TypedBuilder, Serialize)]
pub struct ValidateTemplateRequest { pub subject: Option<String>, pub html_body: Option<String>, ... }
pub struct ValidateTemplateResponse { pub all_content_is_valid: bool, pub subject: ValidationPart, ... }
```

**Step 4: Run test to verify it passes**

Run: `cargo test api::templates::validate_template::tests::validate_template`
Expected: PASS.

**Step 5: Commit**

```bash
git add src/api/templates/validate_template.rs src/api/templates.rs
git commit -m "feat add templates validate endpoint"
```

### Task 3: Webhooks - Remaining CRUD

**Files:**
- Create: `src/api/webhooks/list_webhooks.rs`
- Create: `src/api/webhooks/get_webhook.rs`
- Create: `src/api/webhooks/edit_webhook.rs`
- Create: `src/api/webhooks/delete_webhook.rs`
- Modify: `src/api/webhooks.rs`
- Test: each new file under `#[cfg(test)]`

**Step 1: Write failing tests**

```rust
#[tokio::test] async fn list_webhooks() { /* GET /webhooks?MessageStream=outbound */ }
#[tokio::test] async fn get_webhook() { /* GET /webhooks/{id} */ }
#[tokio::test] async fn edit_webhook() { /* PUT /webhooks/{id} */ }
#[tokio::test] async fn delete_webhook() { /* DELETE /webhooks/{id} */ }
```

**Step 2: Run tests to verify they fail**

Run: `cargo test api::webhooks::`
Expected: FAIL in new modules before implementation.

**Step 3: Write minimal implementation**

```rust
pub struct Webhook { pub id: isize, pub url: String, pub message_stream: String, ... }
pub struct Triggers { pub open: Option<OpenTrigger>, ... }
impl Endpoint for ListWebhooksRequest { ... }
impl Endpoint for GetWebhookRequest { ... }
impl Endpoint for EditWebhookRequest { ... }
impl Endpoint for DeleteWebhookRequest { ... }
```

**Step 4: Run tests to verify they pass**

Run: `cargo test api::webhooks::`
Expected: PASS for list/get/edit/delete/create.

**Step 5: Commit**

```bash
git add src/api/webhooks.rs src/api/webhooks/*.rs
git commit -m "feat add webhooks list get edit delete"
```

### Task 4: Suppressions - Create Suppression

**Files:**
- Create: `src/api/message_streams/create_suppression.rs`
- Modify: `src/api/message_streams.rs`
- Test: `src/api/message_streams/create_suppression.rs`

**Step 1: Write the failing test**

```rust
#[tokio::test]
async fn create_suppression() {
    // Expect POST /message-streams/{id}/suppressions
    // Assert decode statuses Suppressed/Failed
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test api::message_streams::create_suppression::tests::create_suppression`
Expected: FAIL with unresolved module/type.

**Step 3: Write minimal implementation**

```rust
pub enum SuppressionCreateStatusType { Suppressed, Failed }
pub struct CreateSuppressionRequest { pub stream_id: StreamIdOrName, pub suppressions: Vec<Emails> }
```

**Step 4: Run test to verify it passes**

Run: `cargo test api::message_streams::create_suppression::tests::create_suppression`
Expected: PASS.

**Step 5: Commit**

```bash
git add src/api/message_streams/create_suppression.rs src/api/message_streams.rs
git commit -m "feat add suppressions create endpoint"
```

### Task 5: Message Streams - List/Get

**Files:**
- Create: `src/api/message_streams/list_message_streams.rs`
- Create: `src/api/message_streams/get_message_stream.rs`
- Modify: `src/api/message_streams.rs`
- Test: new files

**Step 1: Write failing tests**

```rust
#[tokio::test] async fn list_message_streams() { /* GET /message-streams?... */ }
#[tokio::test] async fn get_message_stream() { /* GET /message-streams/{id} */ }
```

**Step 2: Run tests to verify they fail**

Run: `cargo test api::message_streams::list_message_streams::tests::list_message_streams api::message_streams::get_message_stream::tests::get_message_stream`
Expected: FAIL with unresolved module/type.

**Step 3: Write minimal implementation**

```rust
pub struct MessageStreamDetails { pub id: String, pub server_id: isize, ... }
pub struct ListMessageStreamsResponse { pub message_streams: Vec<MessageStreamDetails>, pub total_count: isize }
```

**Step 4: Run tests to verify they pass**

Run: `cargo test api::message_streams::list_message_streams::tests::list_message_streams api::message_streams::get_message_stream::tests::get_message_stream`
Expected: PASS.

**Step 5: Commit**

```bash
git add src/api/message_streams/list_message_streams.rs src/api/message_streams/get_message_stream.rs src/api/message_streams.rs
git commit -m "feat add message streams list get"
```

### Task 6: Message Streams - Create/Edit/Archive/Unarchive

**Files:**
- Create: `src/api/message_streams/create_message_stream.rs`
- Create: `src/api/message_streams/edit_message_stream.rs`
- Create: `src/api/message_streams/archive_message_stream.rs`
- Create: `src/api/message_streams/unarchive_message_stream.rs`
- Modify: `src/api/message_streams.rs`
- Test: new files

**Step 1: Write failing tests**

```rust
#[tokio::test] async fn create_message_stream() { /* POST /message-streams */ }
#[tokio::test] async fn edit_message_stream() { /* PATCH /message-streams/{id} */ }
#[tokio::test] async fn archive_message_stream() { /* POST /message-streams/{id}/archive */ }
#[tokio::test] async fn unarchive_message_stream() { /* POST /message-streams/{id}/unarchive */ }
```

**Step 2: Run tests to verify they fail**

Run: `cargo test api::message_streams::create_message_stream::tests::create_message_stream api::message_streams::edit_message_stream::tests::edit_message_stream api::message_streams::archive_message_stream::tests::archive_message_stream api::message_streams::unarchive_message_stream::tests::unarchive_message_stream`
Expected: FAIL before implementation.

**Step 3: Write minimal implementation**

```rust
pub enum MessageStreamType { Inbound, Broadcasts, Transactional }
pub struct SubscriptionManagementConfiguration { pub unsubscribe_handling_type: String }
```

**Step 4: Run tests to verify they pass**

Run: `cargo test api::message_streams::`
Expected: PASS for all message stream endpoints.

**Step 5: Commit**

```bash
git add src/api/message_streams/*.rs
git commit -m "feat add message streams write and lifecycle endpoints"
```

### Task 7: Server + Servers Missing Endpoints

**Files:**
- Create: `src/api/server/edit_server.rs`
- Create: `src/api/server/list_servers.rs`
- Create: `src/api/server/delete_server.rs`
- Modify: `src/api/server/get_server.rs` (switch endpoint to `/server`)
- Modify: `src/api/server.rs`
- Test: each new file + update get_server test path

**Step 1: Write failing tests**

```rust
#[tokio::test] async fn get_server() { /* GET /server */ }
#[tokio::test] async fn edit_server() { /* PUT /server */ }
#[tokio::test] async fn list_servers() { /* GET /servers?count=...&offset=... */ }
#[tokio::test] async fn delete_server() { /* DELETE /servers/{id} */ }
```

**Step 2: Run tests to verify they fail**

Run: `cargo test api::server::`
Expected: FAIL on missing modules and old `/servers/{id}` path for current get-server test.

**Step 3: Write minimal implementation**

```rust
impl Endpoint for GetServerRequest { fn endpoint(&self) -> Cow<'static, str> { "/server".into() } }
pub struct ListServersResponse { pub total_count: isize, pub servers: Vec<ServerDetails> }
```

**Step 4: Run tests to verify they pass**

Run: `cargo test api::server::`
Expected: PASS.

**Step 5: Commit**

```bash
git add src/api/server.rs src/api/server/*.rs
git commit -m "feat add server and servers remaining endpoints"
```

### Task 8: Bounce Missing Endpoints

**Files:**
- Create: `src/api/bounce/list_bounces.rs`
- Create: `src/api/bounce/get_bounce.rs`
- Create: `src/api/bounce/get_bounce_dump.rs`
- Create: `src/api/bounce/activate_bounce.rs`
- Modify: `src/api/bounce.rs`
- Test: each new file

**Step 1: Write failing tests**

```rust
#[tokio::test] async fn list_bounces() { /* GET /bounces?count=50&offset=0 */ }
#[tokio::test] async fn get_bounce() { /* GET /bounces/{id} */ }
#[tokio::test] async fn get_bounce_dump() { /* GET /bounces/{id}/dump */ }
#[tokio::test] async fn activate_bounce() { /* PUT /bounces/{id}/activate */ }
```

**Step 2: Run tests to verify they fail**

Run: `cargo test api::bounce::`
Expected: FAIL with unresolved module/type.

**Step 3: Write minimal implementation**

```rust
pub struct BounceRecord { pub id: isize, pub r#type: String, pub message_id: String, ... }
pub struct ListBouncesResponse { pub total_count: isize, pub bounces: Vec<BounceRecord> }
pub struct BounceDumpResponse { pub body: String }
```

**Step 4: Run tests to verify they pass**

Run: `cargo test api::bounce::`
Expected: PASS.

**Step 5: Commit**

```bash
git add src/api/bounce.rs src/api/bounce/*.rs
git commit -m "feat add bounce remaining endpoints"
```

### Task 9: Final Integration Verification

**Files:**
- Modify: none (verification only unless fixes needed)

**Step 1: Run full test suite**

Run: `cargo test`
Expected: PASS all tests.

**Step 2: Run lint**

Run: `cargo clippy --all-targets`
Expected: PASS with no warnings.

**Step 3: Run format**

Run: `cargo fmt`
Expected: no changes, or rerun tests/lint if changes occur.

**Step 4: Commit final cleanup if needed**

```bash
git add -A
git commit -m "chore finalize endpoint coverage rollout"
```
