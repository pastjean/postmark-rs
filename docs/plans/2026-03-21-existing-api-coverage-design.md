# Existing API Coverage Design

## Goal

Add missing Postmark endpoints for API sections already present in this crate, while preserving existing style, ergonomics, and endpoint-per-file structure.

## Scope

In scope: endpoint coverage only for existing section paths/modules.

Out of scope: adding brand-new top-level sections (Messages, Stats, Signatures, Triggers, Data Removal, etc.), broad client refactors, auth-system redesign.

## Current Gap Inventory

Covered now:

- Email: send single, send batch
- Template sending: send with template, batch with templates
- Domains: list/get/create/edit/delete/verify-dkim/verify-return-path/verify-spf/rotate-dkim
- Templates: create/get/edit/delete/push
- Servers: get/create
- Suppressions: dump/delete
- Webhooks: create
- Bounce: delivery stats

Missing endpoints in existing paths:

- Bounce: get bounces, get single bounce, get bounce dump, activate bounce
- Templates: list templates, validate template
- Server API: get `/server`, edit `/server`
- Servers API: edit server, list servers, delete server
- Message Streams: list/get/edit/create/archive/unarchive
- Suppressions: create suppression
- Webhooks: list/get/edit/delete

## Design Principles

1. Keep endpoint-per-file module layout consistent with current codebase.
2. Preserve public API style (`TypedBuilder`, request/response structs, `Endpoint` trait impl).
3. Keep auth responsibility where it is today (token type selected by user/client setup, not endpoint code).
4. Prefer explicit, section-local models over premature shared abstractions.
5. Add tests for wire behavior and decoding for every new endpoint.

## Architecture

### File/Module Pattern

- One endpoint file per operation in existing section modules.
- Re-export from section module (`src/api/<section>.rs`) to keep API discoverable.
- Keep path generation in `endpoint()` and querystring composition close to each request type.

### Models

- Reuse existing response models where exact shape matches.
- Introduce new response structs for endpoint-specific shapes.
- Add section-scoped enums for constrained values only when needed (e.g., filters/status types).

### Query Parameters

- Keep parameters explicit in request struct fields.
- Serialize optional params only when set.
- Follow docs naming/casing exactly for query keys and payload keys.

## Testing Strategy

Per endpoint, add:

1. Request-method/path assertion test (including querystring when applicable).
2. Response decode test using docs-like payload.
3. Request serialization assertion where body or optional fields are non-trivial.

Validation gates after each section batch:

- `cargo fmt`
- `cargo test`
- `cargo clippy --all-targets`

## Rollout Plan (Section by Section)

1. Templates: list + validate
2. Webhooks: list/get/edit/delete
3. Suppressions: create
4. Message Streams: list/get/edit/create/archive/unarchive
5. Server + Servers missing endpoints
6. Bounce missing endpoints

Rationale: quick wins first, then larger model surface, then remaining infra-style endpoints.

## Risks and Mitigations

- Endpoint casing drift vs docs: mitigate with precise path tests.
- Response shape mismatch: mitigate with docs-derived fixture payloads.
- Enum casing mismatch: mitigate with serde rename annotations + decode tests.

## Success Criteria

- All documented endpoints in already-present sections implemented.
- All new endpoints have method/path + decode coverage.
- `cargo fmt`, `cargo test`, `cargo clippy --all-targets` green.
