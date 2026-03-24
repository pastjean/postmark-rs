# API Wrapper Modernization Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Reduce endpoint boilerplate while increasing type safety and consistency via canonical endpoint metadata, shared query/path/error primitives, and strict drift CI.

**Architecture:** Introduce explicit metadata and helper layers first, then migrate a pilot endpoint set, then scale out. Keep endpoint files readable and debuggable; run a measured explicit-vs-proc-macro spike before broad conversion. Lock correctness with parity tests and generated endpoint-map/docs drift gates.

**Tech Stack:** Rust (`serde`, `typed-builder`, `http`, `reqwest`, `thiserror`), cargo test/clippy/doc, GitHub Actions CI.

---

### Task 1: Add Endpoint Drift Parity Test Gate

**Files:**
- Create: `tests/endpoint_parity.rs`
- Create: `tests/support/mod.rs`
- Modify: `Cargo.toml`
- Modify: `src/lib.rs`

**Step 1: Write the failing test**

```rust
#[test]
fn endpoint_map_is_consistent_for_pilot_endpoints() {
    let map = postmark::api::meta::canonical_endpoint_map();
    assert!(map.iter().any(|e| e.path_template == "/servers" && e.method == "GET"));
    assert!(map.iter().any(|e| e.path_template == "/webhooks" && e.method == "GET"));
    assert!(map.iter().any(|e| e.path_template == "/messages/outbound" && e.method == "GET"));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test endpoint_map_is_consistent_for_pilot_endpoints -q`
Expected: FAIL (missing `api::meta` module/functions).

**Step 3: Write minimal implementation**

```rust
pub mod meta;

// in src/api/meta.rs (temporary seed)
pub struct EndpointMetaRow {
    pub method: &'static str,
    pub path_template: &'static str,
}

pub fn canonical_endpoint_map() -> Vec<EndpointMetaRow> {
    vec![]
}
```

**Step 4: Run test to verify it passes/fails for correct reason**

Run: `cargo test endpoint_map_is_consistent_for_pilot_endpoints -q`
Expected: FAIL on missing rows (good red phase).

**Step 5: Commit**

```bash
git add tests/endpoint_parity.rs tests/support/mod.rs Cargo.toml src/lib.rs src/api/meta.rs src/api.rs
git commit -m "test add endpoint parity gate scaffold"
```

### Task 2: Implement Explicit Endpoint Metadata Core

**Files:**
- Create: `src/api/meta.rs`
- Modify: `src/api.rs`
- Modify: `src/api/server/list_servers.rs`
- Modify: `src/api/webhooks/list_webhooks.rs`
- Modify: `src/api/messages/outbound_search.rs`
- Test: `tests/endpoint_parity.rs`

**Step 1: Write the failing test**

```rust
#[test]
fn endpoint_map_has_auth_and_query_body_flags() {
    let map = postmark::api::meta::canonical_endpoint_map();
    let servers = map.iter().find(|m| m.path_template == "/servers").unwrap();
    assert_eq!(servers.auth, "account");
    assert!(servers.has_query);
    assert!(!servers.has_body);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test endpoint_map_has_auth_and_query_body_flags -q`
Expected: FAIL (fields/rows not implemented).

**Step 3: Write minimal implementation**

```rust
pub enum AuthKind { Server, Account, None }

pub struct EndpointMeta {
    pub method: http::Method,
    pub path_template: &'static str,
    pub auth: AuthKind,
    pub has_query: bool,
    pub has_body: bool,
}

pub const LIST_SERVERS_META: EndpointMeta = EndpointMeta {
    method: http::Method::GET,
    path_template: "/servers",
    auth: AuthKind::Account,
    has_query: true,
    has_body: false,
};
```

**Step 4: Run test to verify it passes**

Run: `cargo test endpoint_map_ -q`
Expected: PASS for parity tests.

**Step 5: Commit**

```bash
git add src/api/meta.rs src/api.rs src/api/server/list_servers.rs src/api/webhooks/list_webhooks.rs src/api/messages/outbound_search.rs tests/endpoint_parity.rs
git commit -m "feat add explicit endpoint metadata for pilot endpoints"
```

### Task 3: Add Shared Query Encoder and Fix Drift

**Files:**
- Create: `src/api/query.rs`
- Modify: `src/api.rs`
- Modify: `src/api/server/list_servers.rs`
- Modify: `src/api/webhooks/list_webhooks.rs`
- Modify: `src/api/messages/outbound_search.rs`
- Test: `src/api/server/list_servers.rs`
- Test: `src/api/webhooks/list_webhooks.rs`
- Test: `src/api/messages/outbound_search.rs`

**Step 1: Write the failing tests**

```rust
#[test]
fn list_servers_endpoint_encodes_query_consistently() {
    let req = ListServersRequest::builder().count(100).offset(0).build();
    assert_eq!(req.endpoint(), "/servers?count=100&offset=0");
}
```

```rust
#[test]
fn list_webhooks_message_stream_is_encoded() {
    let req = ListWebhooksRequest::builder().message_stream("broadcast").build();
    assert_eq!(req.endpoint(), "/webhooks?MessageStream=broadcast");
}
```

**Step 2: Run tests to verify at least one fails**

Run: `cargo test list_servers_endpoint_encodes_query_consistently list_webhooks_message_stream_is_encoded -q`
Expected: FAIL before helper migration.

**Step 3: Write minimal implementation**

```rust
pub struct QueryBuilder {
    serializer: url::form_urlencoded::Serializer<'static, String>,
}

impl QueryBuilder {
    pub fn new() -> Self { Self { serializer: url::form_urlencoded::Serializer::new(String::new()) } }
    pub fn push_opt(mut self, key: &str, value: Option<String>) -> Self { if let Some(v) = value { self.serializer.append_pair(key, &v); } self }
    pub fn finish(self) -> String { self.serializer.finish() }
}
```

**Step 4: Run tests to verify they pass**

Run: `cargo test list_servers_endpoint_encodes_query_consistently list_webhooks_message_stream_is_encoded outbound_search_encodes_query_params -q`
Expected: PASS.

**Step 5: Commit**

```bash
git add src/api/query.rs src/api.rs src/api/server/list_servers.rs src/api/webhooks/list_webhooks.rs src/api/messages/outbound_search.rs
git commit -m "refactor unify query encoding in pilot endpoints"
```

### Task 4: Introduce Typed ID and Error Code Newtypes

**Files:**
- Create: `src/api/types.rs`
- Modify: `src/api.rs`
- Modify: `src/api/server.rs`
- Modify: `src/api/domains.rs`
- Modify: `src/api/templates.rs`
- Modify: `src/api/webhooks.rs`
- Modify: `src/api/messages.rs`
- Test: `tests/typed_ids.rs`

**Step 1: Write the failing test**

```rust
#[test]
fn typed_ids_roundtrip_serde() {
    let id = postmark::api::types::ServerId::new(42);
    let json = serde_json::to_string(&id).unwrap();
    assert_eq!(json, "42");
    let back: postmark::api::types::ServerId = serde_json::from_str(&json).unwrap();
    assert_eq!(back.get(), 42);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test typed_ids_roundtrip_serde -q`
Expected: FAIL (types missing).

**Step 3: Write minimal implementation**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct ServerId(i64);

impl ServerId {
    pub const fn new(value: i64) -> Self { Self(value) }
    pub const fn get(self) -> i64 { self.0 }
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test typed_ids_roundtrip_serde -q`
Expected: PASS.

**Step 5: Commit**

```bash
git add src/api/types.rs src/api.rs src/api/server.rs src/api/domains.rs src/api/templates.rs src/api/webhooks.rs src/api/messages.rs tests/typed_ids.rs
git commit -m "feat add typed id primitives"
```

### Task 5: Unify API Error Envelope and Decode Path

**Files:**
- Create: `src/error.rs`
- Modify: `src/lib.rs`
- Modify: `src/client.rs`
- Modify: `src/reqwest.rs`
- Modify: `src/api/templates/delete_template.rs`
- Modify: `src/api/server/delete_server.rs`
- Modify: `src/api/webhooks/delete_webhook.rs`
- Test: `tests/error_model.rs`

**Step 1: Write the failing tests**

```rust
#[tokio::test]
async fn non_success_returns_api_envelope_error() {
    // mock 422 with { ErrorCode, Message }
    // assert Query::execute returns ApiErrorEnvelope variant
}
```

```rust
#[tokio::test]
async fn invalid_json_on_2xx_returns_transport_decode_error() {
    // mock 200 invalid payload
    // assert transport/decode variant, not api envelope
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test non_success_returns_api_envelope_error invalid_json_on_2xx_returns_transport_decode_error -q`
Expected: FAIL (new error model not wired).

**Step 3: Write minimal implementation**

```rust
#[derive(Debug, thiserror::Error)]
pub enum QueryError<E: std::error::Error + Send + Sync + 'static> {
    #[error("transport error: {0}")]
    Transport(#[from] E),
    #[error("decode error: {0}")]
    Decode(#[from] serde_json::Error),
    #[error("api error {http_status} code {error_code}: {message}")]
    Api { http_status: u16, error_code: i64, message: String },
}
```

**Step 4: Run tests to verify they pass**

Run: `cargo test error_model -q`
Expected: PASS.

**Step 5: Commit**

```bash
git add src/error.rs src/lib.rs src/client.rs src/reqwest.rs src/api/templates/delete_template.rs src/api/server/delete_server.rs src/api/webhooks/delete_webhook.rs tests/error_model.rs
git commit -m "feat unify api envelope vs transport errors"
```

### Task 6: Run Explicit vs Proc-Macro Spike and Decide

**Files:**
- Create: `src/api/spike/explicit.rs`
- Create: `src/api/spike/proc_macro.rs`
- Create: `postmark-macros/Cargo.toml`
- Create: `postmark-macros/src/lib.rs`
- Create: `tests/spike_compare.rs`
- Create: `docs/plans/2026-03-23-api-wrapper-macro-eval.md`

**Step 1: Write the failing comparison test**

```rust
#[test]
fn explicit_and_macro_tracks_produce_identical_meta() {
    assert_eq!(explicit::meta_rows(), proc_macro_track::meta_rows());
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test explicit_and_macro_tracks_produce_identical_meta -q`
Expected: FAIL until both tracks implemented.

**Step 3: Write minimal implementation**

```rust
// explicit track + derive/attr macro track for same 3 endpoints
// both expose meta_rows() for objective compare
```

**Step 4: Run tests and record scorecard**

Run: `cargo test spike_compare -q`
Expected: PASS.

Run: `cargo check -q`
Expected: PASS.

Document scorecard in `docs/plans/2026-03-23-api-wrapper-macro-eval.md` with measured counts and final decision.

**Step 5: Commit**

```bash
git add src/api/spike/explicit.rs src/api/spike/proc_macro.rs postmark-macros/Cargo.toml postmark-macros/src/lib.rs tests/spike_compare.rs docs/plans/2026-03-23-api-wrapper-macro-eval.md
git commit -m "spike compare explicit vs proc-macro endpoint wiring"
```

### Task 7: Apply Chosen Pattern to All Endpoints + Generate Docs/Map

**Files:**
- Modify: `src/api/**/*.rs` (endpoint modules)
- Create: `scripts/generate-endpoint-map.rs`
- Create: `docs/api/generated-endpoint-map.json`
- Modify: `docs/api/postmark-endpoints.md`
- Test: `tests/endpoint_parity.rs`

**Step 1: Write the failing drift test**

```rust
#[test]
fn generated_map_matches_checked_in_json() {
    // generate from code
    // compare exact JSON bytes with docs/api/generated-endpoint-map.json
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test generated_map_matches_checked_in_json -q`
Expected: FAIL before generation/check-in sync.

**Step 3: Write minimal implementation**

```rust
// script/test emits stable sorted rows:
// method,path_template,auth,has_query,has_body,request_type
```

**Step 4: Run tests to verify they pass**

Run: `cargo test endpoint_parity -q`
Expected: PASS.

Run: `cargo test -q`
Expected: PASS.

**Step 5: Commit**

```bash
git add src/api scripts/generate-endpoint-map.rs docs/api/generated-endpoint-map.json docs/api/postmark-endpoints.md tests/endpoint_parity.rs
git commit -m "refactor apply chosen endpoint pattern and lock map parity"
```

### Task 8: Harden CI + Transport Readability Improvements

**Files:**
- Modify: `.github/workflows/ci.yml`
- Modify: `Cargo.toml`
- Modify: `src/reqwest.rs`
- Modify: `README.md`

**Step 1: Write the failing quality gate locally**

Run: `cargo clippy --all-targets --all-features -- -D warnings`
Expected: FAIL until warnings addressed.

Run: `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features`
Expected: FAIL if docs warnings exist.

**Step 2: Implement minimal fixes**

```rust
// reqwest.rs:
// - reuse reqwest::Client in PostmarkClient struct
// - remove unwrap in URI parse path
// - map URI parse failure into typed error
```

**Step 3: Verify all quality gates**

Run: `cargo fmt --all -- --check`
Expected: PASS.

Run: `cargo clippy --all-targets --all-features -- -D warnings`
Expected: PASS.

Run: `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features`
Expected: PASS.

Run: `cargo test --all-features -q`
Expected: PASS.

**Step 4: Commit**

```bash
git add .github/workflows/ci.yml Cargo.toml src/reqwest.rs README.md
git commit -m "chore enforce strict ci gates and reqwest cleanup"
```

### Task 9: Final Verification + Change Log

**Files:**
- Modify: `README.md`
- Modify: `CHANGELOG.md` (create if absent)

**Step 1: Verify end-to-end**

Run: `cargo test --all-features`
Expected: PASS.

Run: `cargo clippy --all-targets --all-features -- -D warnings`
Expected: PASS.

Run: `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features`
Expected: PASS.

**Step 2: Document breaking changes**

```markdown
## Breaking changes
- typed id newtypes replace raw integer ids in public structs
- query/error type updates
```

**Step 3: Commit**

```bash
git add README.md CHANGELOG.md
git commit -m "docs record modernization changes and breakage"
```
