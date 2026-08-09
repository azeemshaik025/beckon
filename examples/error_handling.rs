//! Example demonstrating how to inspect a failed request.
//!
//! On any non-2xx response, the generated error's `Http` variant carries the raw
//! response body the server returned — usually a JSON error object — so you can see
//! *why* a request was rejected, not just the status code.

use beckon::beckon;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct User {
    id: u32,
    name: String,
}

#[derive(Serialize)]
pub struct UserPath {
    id: u32,
}

beckon!(
    ApiClient,
    {
        {
            path: "/users/{id}",
            method: GET,
            path_params: UserPath,
            res: User,
        },
    }
);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ApiClient::new(
        reqwest::Url::parse("https://api.example.com")?,
        Duration::from_secs(5),
    );

    match client.get_users_by_id(&UserPath { id: 999 }).await {
        Ok(user) => println!("got user: {user:?}"),

        // The server explained the failure in the response body — now it's yours to
        // read, log, or parse, instead of being discarded behind a bare status code.
        Err(ApiClientError::Http {
            status,
            reason,
            body,
        }) => {
            eprintln!("request failed: HTTP {status} {reason}");
            eprintln!("server said: {body}"); // e.g. {"error":"user not found"}
        }

        Err(other) => eprintln!("transport or decode error: {other}"),
    }

    Ok(())
}
