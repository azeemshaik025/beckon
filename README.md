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

Requires **Rust 1.75+** — the generated mocking trait uses `async fn` in traits.

## Example

```rust,no_run
use beckon::beckon;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
}

#[derive(Serialize)]
pub struct UserPath {
    pub id: u32,
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
        Duration::from_secs(5),
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

Method names are derived from the method and path — `GET /users` → `get_users`,
`GET /users/{id}` → `get_users_by_id`. Set `fn_name` to override.

## Auth

Add automatic authentication to every request. Three strategies are supported:

```rust,ignore
// Bearer token — injects `Authorization: Bearer <token>`
beckon!(GithubApi, auth: Bearer, { /* ... */ });
let client = GithubApi::new(url, "ghp_xxxx", Duration::from_secs(5));

// Basic auth — injects `Authorization: Basic <base64>`
beckon!(DbApi, auth: Basic, { /* ... */ });
let client = DbApi::new(url, "admin", "secret", Duration::from_secs(5));

// API key — injects a custom header
beckon!(StripeApi, auth: ApiKey("X-Api-Key"), { /* ... */ });
let client = StripeApi::new(url, "sk_live_xxxx", Duration::from_secs(5));
```

Omitting `auth` keeps the plain `new(url, timeout)` constructor. Auth composes with every
other feature, including retries.

## Retry with Backoff

Set a global retry count that applies to all endpoints. Retries use exponential backoff
(100ms base, 2x multiplier, 5s cap) and trigger on 5xx errors and request timeouts. 4xx
errors are never retried.

```rust,ignore
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

## Constructors

- `UserApi::new(url, timeout)` — uses a default `reqwest::Client`.
- `UserApi::with_client(url, client, timeout)` — supply your own `reqwest::Client` to
  share a connection pool, TLS config, proxy, or default headers.

`timeout` accepts a `std::time::Duration`, or `None` for the 5-second default.

```rust,ignore
let http = reqwest::Client::builder().user_agent("my-app/1.0").build()?;
let client = UserApi::with_client(url, http, Duration::from_secs(5));
```

## Errors

Every method returns `Result<Res, UserApiError>`. On a non-2xx response you get the
`Http` variant, which carries the server's response body so you can see *why* a request
was rejected:

```rust,ignore
match client.get_users_by_id(&UserPath { id: 999 }).await {
    Ok(user) => { /* ... */ }
    Err(UserApiError::Http { status, reason, body }) => {
        eprintln!("HTTP {status} {reason}: {body}"); // body = the server's error payload
    }
    Err(other) => eprintln!("{other}"),
}
```

The enum is `#[non_exhaustive]`, so matching downstream should keep a `_` arm.

## Generated Code

For a client named `UserApi`, the macro generates:

- A **struct** `UserApi` (derives `Clone`) with `new` and `with_client` constructors
- An **async method** for each endpoint
- A **trait** `UserApiTrait` for mocking in tests
- An **error enum** `UserApiError` with variants for URL, request, HTTP, and deserialization errors

## Examples

See the [`examples/`](https://github.com/azeemshaik025/beckon/tree/main/examples) directory:

- [`basic.rs`](https://github.com/azeemshaik025/beckon/blob/main/examples/basic.rs) — simple GET requests
- [`params.rs`](https://github.com/azeemshaik025/beckon/blob/main/examples/params.rs) — path and query parameters
- [`advanced.rs`](https://github.com/azeemshaik025/beckon/blob/main/examples/advanced.rs) — all features
- [`mocking.rs`](https://github.com/azeemshaik025/beckon/blob/main/examples/mocking.rs) — testing with the generated trait
- [`multiple_path_params.rs`](https://github.com/azeemshaik025/beckon/blob/main/examples/multiple_path_params.rs) — nested resources
- [`error_handling.rs`](https://github.com/azeemshaik025/beckon/blob/main/examples/error_handling.rs) — inspecting a failed request's body

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option.
