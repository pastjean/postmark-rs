# Postmark API parity design

## Context

- Goal: implement full Postmark public API endpoint coverage in this crate.
- Current crate has partial coverage across `email`, `templates`, `server`, `webhooks`, `message_streams`, `bounce`.
- User requested one big pass (single large implementation cycle).

## Objectives

- Add typed endpoint bindings for all currently documented Postmark public API endpoints.
- Keep request/response typing consistent with current crate patterns.
- Ensure docs reflect real code coverage with zero silent gaps.
- Keep client transport and error model stable.

## Scope definition

Parity means one of the following for each public endpoint:

- Implemented as a typed endpoint request/response in this crate, exported in the proper module.
- Or explicitly listed as intentionally unsupported in documentation with reason.

## Architecture and module strategy

- Keep existing architecture: one endpoint per file under `src/api/<group>/`.
- Keep API group modules in `src/api/*.rs` as public entry points.
- Add missing API groups if required by Postmark docs (domain decided by endpoint inventory).
- Keep `Client`, `Query`, and reqwest transport implementation unchanged.

## Endpoint implementation pattern

Per endpoint:

- Define `*Request` struct using typed-builder and serde mappings.
- Define `*Response` struct(s) with serde-compatible field names and enums.
- Implement `Endpoint` with exact method/path/body behavior.
- Re-export endpoint types in the group module.

Conventions:

- Keep existing naming style (`GetXRequest`, `CreateXRequest`, `DeleteXRequest`, `ListXRequest`).
- Use shared types only when reused in at least two endpoints.
- Use explicit serde renames and enum mappings for wire compatibility.

## Docs parity strategy

- Add a coverage table in `README.md` mapping endpoint groups and implementation status.
- Update module-level docs so users can discover implemented endpoints quickly.
- Add focused examples per major group (not every endpoint) to keep docs concise.
- If any endpoint remains unsupported, list it explicitly in docs with rationale.

## Verification strategy

- Add endpoint-level tests for:
  - request serialization shape,
  - endpoint path and method,
  - representative response deserialization.
- Prefer fast unit tests; integration tests only where existing harness already supports it.
- Final parity gate:
  - endpoint matrix has no undocumented gaps,
  - tests pass,
  - docs table and exports are synchronized.

## Error handling and compatibility

- Do not change `QueryError` or `PostmarkClientError` semantics as part of parity rollout.
- Keep backward compatibility for existing endpoints and public types.
- Avoid speculative refactors unrelated to endpoint parity.

## Delivery shape

Single large branch/PR, executed in ordered internal phases:

1. Build endpoint inventory baseline against current Postmark docs.
2. Implement missing endpoints by domain.
3. Update README/module docs and examples for parity visibility.
4. Run full verification and finalize parity report.

## Risks and mitigations

- Risk: API docs drift during implementation.
  - Mitigation: refresh inventory before final verification.
- Risk: inconsistent field naming across domains.
  - Mitigation: enforce serde-based wire compatibility tests per endpoint.
- Risk: oversized review surface.
  - Mitigation: maintain strict per-domain commit structure inside single PR.
