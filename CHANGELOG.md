# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0]

This release contains breaking changes to the constructor signature and the generated
error enum.

### Added

- **The `Http` error variant now carries the response body.** A non-2xx response keeps the
  raw payload the server returned (`Http { status, reason, body }`), so the reason a request
  was rejected is no longer discarded. `Display` includes the body when present.

### Changed

- **Breaking:** `new` and `with_client` now take the timeout as
  `impl Into<Option<std::time::Duration>>` instead of `Option<u64>` milliseconds. Pass a
  `Duration` (e.g. `Duration::from_secs(5)`) or `None` for the 5-second default. This removes
  the footgun where `Some(30)` meant 30 **milliseconds**, not 30 seconds.
- **Breaking:** the generated error enum is now `#[non_exhaustive]` and the `Http` variant has
  a new `body` field. Exhaustive `match`es on it (from another crate) need a `_` arm.

## [0.2.0]

### Fixed

- **Path parameters are now percent-encoded.** A value such as `1/2` or `a?b` is
  encoded (`1%2F2`, `a%3Fb`) so it can no longer break out of its URL path segment.
  Previously it was substituted verbatim.

### Added

- `with_client(url, [auth], client, timeout)` constructor, so a caller-supplied
  `reqwest::Client` — and its connection pool, TLS config, proxy, or default headers —
  can be shared across clients. `new` now builds on top of it.
- The generated client struct derives `Clone` (cheap: `reqwest::Client` is `Arc`-backed),
  so it can be cloned into spawned tasks.

## [0.1.1]

### Documentation

- The crate-level example is now a compile-tested doctest instead of `ignore`,
  so it is verified against the public API on every build.

## [0.1.0]

Initial release of `beckon`.

`beckon` is the continuation of the crate previously published as `http-provider-macro`,
renamed for a clearer, single, consistent name across the macro, crate, and repository.

### Features

- `beckon!` macro that generates a type-safe, async HTTP client from endpoint definitions.
- Per-endpoint configuration: `method`, `path`, `req`, `res`, `path_params`, `query_params`,
  `headers`, `fn_name`, and `retries`.
- Built-in auth strategies: `Bearer`, `Basic`, and `ApiKey`.
- Global and per-endpoint retries with exponential backoff (5xx and timeouts only).
- A generated trait per client for mocking in tests, and a typed error enum.
