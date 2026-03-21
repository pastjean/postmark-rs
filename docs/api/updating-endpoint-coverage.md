# Updating endpoint coverage

Use this flow when Postmark docs change.

1. Compare sections in Postmark API reference against `docs/api/postmark-endpoints.md`.
2. For every missing endpoint, add one endpoint file under the section module.
3. Add re-export in section mod file and `src/api.rs` if it is a new section.
4. Add tests per endpoint:
   - method/path matcher test
   - response decode test
   - request serialization/query assertion (when relevant)
5. Run:
   - `cargo fmt`
   - `cargo test`
   - `cargo clippy --all-targets`
6. Update endpoint matrix and examples in docs.

## Review checklist

- path casing matches docs exactly
- token type documented (server/account)
- deprecated endpoint policy respected
- all new modules exported publicly
