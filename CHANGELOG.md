# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.11.4](https://github.com/pastjean/postmark-rs/compare/v0.11.3...v0.11.4) - 2025-08-07

### Other

- Upgrade thiserror and typed-builder ([#43](https://github.com/pastjean/postmark-rs/pull/43))
- Update README.md
# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.0.0](https://github.com/pastjean/postmark-rs/compare/v1.0.0...v2.0.0) - 2026-05-01

### Other

- release v2.0.0 ([#57](https://github.com/pastjean/postmark-rs/pull/57))
- Add serialization for error responses by postmark for the /email/bulk endpoint ([#56](https://github.com/pastjean/postmark-rs/pull/56))

## [2.0.0](https://github.com/pastjean/postmark-rs/compare/v1.0.0...v2.0.0) - 2026-05-01

### Other

- Add serialization for error responses by postmark for the /email/bulk endpoint ([#56](https://github.com/pastjean/postmark-rs/pull/56))

## [1.0.0](https://github.com/pastjean/postmark-rs/compare/v0.11.5...v1.0.0) - 2026-03-25

### Added

- Implement all current top-level Postmark REST API reference sections documented by Postmark, with Sender Signatures limited to non-deprecated endpoints.
- Add new public modules for `bulk`, `data_removal`, `messages`, `signatures`, `stats`, `triggers`, and `types`.
- Add request and response coverage for Bulk, Bounce, Message Streams, Suppressions, Messages, Server/Servers, Sender Signatures, Stats, Triggers, Webhooks, Templates, and Data Removal APIs.
- Add API coverage docs, compatibility notes, and examples under `docs/api/`.

### Changed

- Mark `postmark` 1.0.0 as the first stable release of the crate.

### Breaking

- `PostmarkClient` is no longer constructible with a struct literal.
- `PostmarkClient` no longer implements `UnwindSafe` and `RefUnwindSafe`.
- `QueryError` gained `Api` and `PostmarkClientError` gained `InvalidUri`, so exhaustive matches may need updating.
- `DeliveryStatsRequest` and `DeliveryStatsResponse` were renamed to `GetDeliveryStatsRequest` and `GetDeliveryStatsResponse`.
- `CreateServerResponse` and `GetServerResponse` were consolidated into `api::server::Server`.

## [0.11.5](https://github.com/pastjean/postmark-rs/compare/v0.11.4...v0.11.5) - 2026-03-21

### Fixed

- fix clippy warnings in api modules
- fix domains endpoint casing + SPF response shape

### Other

- Change IDs from i64 to isize to match the rest of the codebase
- Add functionality for domains

## [0.11.3](https://github.com/pastjean/postmark-rs/compare/v0.11.2...v0.11.3) - 2025-04-28

### Fixed

- fix missing closing parenthesis in readme
- fix the license readme button link

### Other

- Make sure the license mit 'or' apache 2.0 is clear

## [0.11.2](https://github.com/pastjean/postmark-rs/compare/v0.11.1...v0.11.2) - 2025-04-08

### Other

- Add error_for_status fn ([#37](https://github.com/pastjean/postmark-rs/pull/37))

## [0.11.1](https://github.com/pastjean/postmark-rs/compare/v0.11.0...v0.11.1) - 2025-01-27

### Other

- Fix Readme example ([#35](https://github.com/pastjean/postmark-rs/pull/35))

## [0.11.0](https://github.com/pastjean/postmark-rs/compare/v0.10.2...v0.11.0) - 2024-09-03

### Other
- Add functionality for servers, templates, webhooks ([#32](https://github.com/pastjean/postmark-rs/pull/32))
- Update actions checkout ([#29](https://github.com/pastjean/postmark-rs/pull/29))

## [0.10.2](https://github.com/pastjean/postmark-rs/compare/v0.10.1...v0.10.2) - 2024-07-29

### Other
- Implement send batch email with templates ([#27](https://github.com/pastjean/postmark-rs/pull/27))

## [0.10.1](https://github.com/pastjean/postmark-rs/compare/v0.10.0...v0.10.1) - 2024-06-21

### Other
- Update dependencies to latest version from 2024-06-21

## [0.10.0](https://github.com/pastjean/postmark-rs/compare/v0.9.2...v0.10.0) - 2023-11-21

### Other
- Update dependencies ([#23](https://github.com/pastjean/postmark-rs/pull/23))

## [0.9.2](https://github.com/pastjean/postmark-rs/compare/v0.9.1...v0.9.2) - 2023-09-06

### Other
- Add TLS to test dependencies

## [0.9.1](https://github.com/pastjean/postmark-rs/compare/v0.9.0...v0.9.1) - 2023-09-05

### Other
- Return send email with template to exported status ([#21](https://github.com/pastjean/postmark-rs/pull/21))

## [0.9.0](https://github.com/pastjean/postmark-rs/compare/v0.8.1...v0.9.0) - 2023-08-31

### Other
- Add a manual test (that is skipped) ([#17](https://github.com/pastjean/postmark-rs/pull/17))
- Implement edit and create template endpoints ([#13](https://github.com/pastjean/postmark-rs/pull/13))
- Update README.md ([#14](https://github.com/pastjean/postmark-rs/pull/14))

## [0.8.1](https://github.com/pastjean/postmark-rs/compare/v0.8.0...v0.8.1) - 2023-06-14

### Other
- cargo features and clippy happiness
- new cargo.toml features && info on release-plz
- Add release-plz as a auto releaser
