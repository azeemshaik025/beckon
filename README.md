# beckon

[![crates.io](https://img.shields.io/crates/v/beckon.svg)](https://crates.io/crates/beckon)
[![docs.rs](https://img.shields.io/docsrs/beckon)](https://docs.rs/beckon)
[![CI](https://github.com/azeemshaik025/beckon/actions/workflows/ci.yml/badge.svg)](https://github.com/azeemshaik025/beckon/actions/workflows/ci.yml)
[![license](https://img.shields.io/crates/l/beckon.svg)](#license)

**Generate type-safe, async HTTP clients from endpoint definitions.**

`beckon!` takes a client name and a list of endpoints, and expands to a struct with one
async method per endpoint — plus a trait for mocking and a typed error enum. No hand-written
request boilerplate.

## Install

```sh
cargo add beckon
```

You'll also need the runtime dependencies the generated code uses:

```sh
cargo add reqwest --features json
cargo add serde --features derive
cargo add tokio --features full
```

## Example

```rust
use beckon::beckon;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
}

#[derive(Serialize)]
struct UserPath {
    id: u32,
}

beckon!(
    UserApi,
    {
        {
            path: "/users",
            method: GET,
            res: Vec<User>,
        },
        {
            path: "/users/{id}",
            method: GET,
            path_params: UserPath,
            res: User,
        },
        {
            path: "/users",
            method: POST,
            req: User,
            res: User,
        },
    }
);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = UserApi::new(
        reqwest::Url::parse("https://api.example.com")?,
        Some(5000),
    );

    let users = client.get_users().await?;
    let user = client.get_users_by_id(&UserPath { id: 1 }).await?;

    Ok(())
}
```

## Endpoint Fields

**Required:**

- `method`: HTTP method (`GET`, `POST`, `PUT`, `DELETE`, `PATCH`)

**Optional:**

- `path`: URL path (e.g., `"/users/{id}"`)
- `res`: Response type (defaults to `()`)
- `req`: Request body type
- `path_params`: Type for path parameters
- `query_params`: Type for query parameters
- `headers`: Header type (e.g., `reqwest::header::HeaderMap`)
- `fn_name`: Custom method name
- `retries`: Retry count for this endpoint (overrides the global setting)

## Auth

Add automatic authentication to every request. Three strategies are supported:

```rust
// Bearer token — injects `Authorization: Bearer <token>`
beckon!(GithubApi, auth: Bearer, { /* ... */ });
let client = GithubApi::new(url, "ghp_xxxx", Some(5000));

// Basic auth — injects `Authorization: Basic <base64>`
beckon!(DbApi, auth: Basic, { /* ... */ });
let client = DbApi::new(url, "admin", "secret", Some(5000));

// API key — injects a custom header
beckon!(StripeApi, auth: ApiKey("X-Api-Key"), { /* ... */ });
let client = StripeApi::new(url, "sk_live_xxxx", Some(5000));
```

Omitting `auth` keeps the plain `new(url, timeout)` constructor. Auth composes with every
other feature, including retries.

## Retry with Backoff

Set a global retry count that applies to all endpoints. Retries use exponential backoff
(100ms base, 2x multiplier, 5s cap) and trigger on 5xx errors and request timeouts. 4xx
errors are never retried.

```rust
beckon!(
    UserApi,
    retries: 3,
    {
        {
            path: "/users",
            method: GET,
            res: Vec<User>,
            // inherits retries: 3 from the global setting
        },
        {
            path: "/health",
            method: GET,
            retries: 0, // override: no retries for this endpoint
        },
    }
);
```

Per-endpoint `retries` overrides the global value. Omitting `retries` entirely means no retries.

## Generated Code

For a client named `UserApi`, the macro generates:

- A **struct** `UserApi` with a `new(url, timeout)` constructor
- An **async method** for each endpoint
- A **trait** `UserApiTrait` for mocking in tests
- An **error enum** `UserApiError` with variants for URL, request, HTTP, and deserialization errors

## Examples

See the [`examples/`](examples/) directory:

- [`basic.rs`](examples/basic.rs) — simple GET requests
- [`params.rs`](examples/params.rs) — path and query parameters
- [`advanced.rs`](examples/advanced.rs) — all features
- [`mocking.rs`](examples/mocking.rs) — testing with the generated trait
- [`multiple_path_params.rs`](examples/multiple_path_params.rs) — nested resources

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option.
