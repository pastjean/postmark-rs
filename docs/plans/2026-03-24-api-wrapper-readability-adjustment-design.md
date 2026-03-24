# API wrapper readability adjustment design

## Context

- User rejected pilot endpoint migration style (`meta/query` helper wiring in endpoint files).
- Target: keep endpoint modules explicit and readable, continue modernization on typed primitives, transport, errors, CI.

## Decision

- Revert pilot endpoint files to previous explicit/manual endpoint style.
- Remove endpoint metadata/query abstraction modules introduced for pilot (`src/api/meta.rs`, `src/api/query.rs`).
- Keep accepted modernization work:
  - Rust 2024 edition
  - reqwest client reuse + no URI parse unwrap
  - typed primitive scaffold in `src/api/types.rs`

## Scope

### Remove

- `src/api/meta.rs`
- `src/api/query.rs`
- `tests/endpoint_parity.rs`
- `pub mod meta;` and `pub mod query;` from `src/api.rs`

### Revert to old style

- `src/api/messages/outbound_search.rs`
- `src/api/webhooks/list_webhooks.rs`
- `src/api/server/list_servers.rs`

### Keep

- `src/api/types.rs`
- `tests/typed_ids.rs`
- `Cargo.toml` edition 2024
- `src/reqwest.rs` cleanup

## Rationale

- Endpoint files are the main reading surface for maintainers.
- Explicit `endpoint()`/query building in each file is easier to scan and debug than abstraction layers in this repo.
- Modernization still progresses in high-value areas that do not reduce readability.

## Next refactor direction

- Unify API error envelope models without hiding endpoint behavior.
- Reduce duplicate local enums/types in shared modules with explicit exports.
- Add CI strictness and docs parity checks via scripts/tests, not endpoint macro/meta abstractions.
