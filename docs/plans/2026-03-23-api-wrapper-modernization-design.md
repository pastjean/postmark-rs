# API wrapper modernization design

## Context

- Goal: reduce endpoint boilerplate, keep strong typing, align with newer Rust API-wrapper patterns.
- Constraint: breaking changes allowed.
- Constraint: readability/debuggability prioritized; proc-macro is optional, must be justified by measured benefit.

## Current gaps observed

- Boilerplate repeated across endpoint files for method/path/body/query serialization.
- Query key casing/path style drift exists across modules.
- Error envelope modeling duplicated and inconsistent (`isize`/`i64`, local structs).
- `reqwest::Client` recreated per request in transport path.
- CI does not enforce strict warnings/doc link checks or endpoint drift checks.

## Objectives

- One canonical endpoint metadata source for method/path/auth/query/body.
- Strongly typed core primitives (`ServerId`, `DomainId`, `MessageId`, etc.).
- Unified error model separating transport failures vs Postmark API envelope.
- Lower boilerplate per endpoint without harming readability/debugging.
- CI gates that fail on endpoint drift and warnings.

## Design

### 1) Canonical endpoint metadata

- Add `api/meta` with `EndpointMeta`:
  - `method`
  - `path_template` (canonical `{id}` placeholders)
  - `auth` (`ServerToken`, `AccountToken`, `None`)
  - `query_kind` / `has_body`
- Each endpoint exposes one `const META: EndpointMeta` near request/response types.
- Endpoint docs table and map JSON are generated from this metadata.

### 2) Typed primitives

- Add `api/types` newtypes for ids/codes:
  - `ServerId`, `DomainId`, `TemplateId`, `WebhookId`, `MessageId`, `ErrorCode`.
- Use `serde(transparent)` and `#[repr(transparent)]`.
- Replace mixed raw integer id/code fields incrementally.

### 3) Shared query/path helpers

- Add shared query serializer helper (single behavior for casing/ordering/encoding).
- Add typed path-template expansion helper to avoid ad-hoc `format!` variance.
- Migrate query endpoints to helper to remove manual serializer duplication.

### 4) Unified error model

- Introduce:
  - `TransportError` for http/io/serde framing concerns.
  - `ApiErrorEnvelope { http_status, error_code, message, errors: Option<Vec<_>> }`.
- Centralize response decode path with consistent status handling.
- Ensure non-2xx endpoints parse envelope consistently where applicable.

### 5) Boilerplate reduction evaluation (explicit vs proc-macro)

- Run 2-track spike on 3 endpoints:
  - `messages/outbound_search`
  - `webhooks/list_webhooks`
  - `server/list_servers`
- Track A: explicit Rust (`const META` + plain impl/helper use).
- Track B: micro proc-macro for endpoint metadata/impl generation.
- Scorecard:
  - endpoint LOC + review diff size
  - debug/compiler error clarity
  - rust-analyzer navigation quality
  - safety against method/path/auth/query drift
  - effort to add one new endpoint
- Decision gate: adopt proc-macro only if material boilerplate reduction with no meaningful readability/debuggability regression.

## Testing strategy

- Per endpoint:
  - one serde response test,
  - one path/query construction test.
- Shared `tests/support` fixtures/helpers for server setup and common payload patterns.
- Add endpoint-map parity test validating metadata output against generated artifacts.
- Add error-path tests for transport vs API-envelope separation.

## CI strategy

- Add strict lint/doc gates:
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features`
- Add endpoint drift gate:
  - regenerate map/docs,
  - fail if dirty working tree after generation.
- Keep jobs split for speed (`check`, `test`, `lint`, `docs`, `parity`).

## Migration phases

1. Fix known endpoint drift points (method/path/query casing mismatch) first.
2. Introduce typed ids + shared query/path helpers.
3. Introduce unified error model and central decode path.
4. Run 2-track boilerplate reduction spike, decide explicit vs proc-macro.
5. Convert remaining endpoints using selected pattern.
6. Enable strict CI gates and remove legacy duplicate structures.

## Non-goals

- Full OpenAPI-first generation in this pass.
- Unrelated transport/backend client redesign.

## Risks and mitigations

- Risk: migration churn across many endpoint files.
  - Mitigation: phase by module with parity tests in each step.
- Risk: proc-macro decreases debuggability.
  - Mitigation: objective spike scorecard and explicit decision gate.
- Risk: accidental public API instability beyond intended break scope.
  - Mitigation: track breaking changes in changelog and gate by focused review.
