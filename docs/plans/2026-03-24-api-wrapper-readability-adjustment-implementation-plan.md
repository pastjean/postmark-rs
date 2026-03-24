# API Wrapper Readability Adjustment Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Remove pilot abstraction layers, restore explicit endpoint style, keep accepted modernization improvements.

**Architecture:** Revert only the endpoint abstraction experiment (`meta/query`) and pilot usages. Preserve typed-id and transport improvements already accepted. Validate via full test/lint/doc gates.

**Tech Stack:** Rust, cargo test, clippy, rustdoc, git, GitHub CLI.

---

### Task 1: Remove abstraction modules and references

**Files:**
- Delete: `src/api/meta.rs`
- Delete: `src/api/query.rs`
- Modify: `src/api.rs`
- Delete: `tests/endpoint_parity.rs`

**Step 1: Write failing compile expectation**

Run: `cargo check`
Expected: FAIL after deleting modules but before endpoint file reverts.

**Step 2: Remove files and module refs**

Delete both files and remove `pub mod meta; pub mod query;` exports.

**Step 3: Verify expected failure**

Run: `cargo check`
Expected: FAIL only on pilot endpoints still importing removed modules.

**Step 4: Commit**

```bash
git add src/api.rs
git rm src/api/meta.rs src/api/query.rs tests/endpoint_parity.rs
git commit -m "refactor drop pilot meta query abstraction"
```

### Task 2: Revert pilot endpoint files to explicit style

**Files:**
- Modify: `src/api/messages/outbound_search.rs`
- Modify: `src/api/webhooks/list_webhooks.rs`
- Modify: `src/api/server/list_servers.rs`

**Step 1: Restore manual endpoint/query code**

Revert imports/constants using `meta/query`; restore local explicit query build logic.

**Step 2: Run targeted tests**

Run: `cargo test outbound_search_encodes_query_params list_webhooks_message_stream_is_encoded list_servers_endpoint_encodes_query_consistently`
Expected: PASS.

**Step 3: Commit**

```bash
git add src/api/messages/outbound_search.rs src/api/webhooks/list_webhooks.rs src/api/server/list_servers.rs
git commit -m "refactor restore explicit pilot endpoint wiring"
```

### Task 3: Full verification and PR update

**Files:**
- Modify: `docs/plans/2026-03-24-api-wrapper-readability-adjustment-design.md`
- Modify: `docs/plans/2026-03-24-api-wrapper-readability-adjustment-implementation-plan.md`

**Step 1: Run full gates**

Run: `cargo test`
Expected: PASS.

Run: `cargo clippy --all-targets --all-features -- -D warnings`
Expected: PASS.

Run: `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features`
Expected: PASS.

**Step 2: Commit docs update**

```bash
git add docs/plans/2026-03-24-api-wrapper-readability-adjustment-design.md docs/plans/2026-03-24-api-wrapper-readability-adjustment-implementation-plan.md
git commit -m "docs add readability adjustment design and plan"
```

**Step 3: Push and update PR**

Run:

```bash
git push
gh pr comment <PR_NUMBER> --body "Rolled back pilot endpoint abstraction (meta/query), restored explicit endpoint style, kept typed ids + transport + edition upgrades."
```
