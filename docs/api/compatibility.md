# API compatibility notes

## Token model

- Endpoints that require `X-Postmark-Server-Token` should be used with `PostmarkClient.server_token`.
- Endpoints that require `X-Postmark-Account-Token` should be used with `PostmarkClient.account_token`.
- The client can carry both; endpoint docs in `docs/api/postmark-endpoints.md` show expected token type.

## Deprecated endpoints policy

- Sender Signature deprecated endpoints are intentionally excluded:
  - `POST /senders/{id}/verifyspf`
  - `POST /senders/{id}/requestnewdkim`
- Preferred replacement for DKIM operations is Domains API (`/domains/{id}/rotatedkim`).

## Field naming and casing

- API payloads are modeled with Postmark PascalCase keys through serde renames.
- Paths are matched to docs exactly (including lowercase segments like `verifyspf`, `rotatedkim`).

## Forward-compat strategy

- Unknown fields from Postmark are ignored by serde by default.
- Dynamic-key stats endpoints (`emailclients`, `browserfamilies`) are represented with a flattened map payload in response structs.
