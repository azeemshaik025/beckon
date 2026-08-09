//! Basic example demonstrating minimal usage of the `beckon` macro.
//!
//! This example shows the simplest way to create an HTTP API client
//! with just the essential fields: `method`, `path`, and `res`.

use beckon::beckon;
use reqwest::Url;
use serde::Deserialize;
use std::time::Duration;

// Define your response types
#[derive(Deserialize, Debug)]
pub struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Deserialize, Debug)]
pub struct Post {
    id: u32,
    title: String,
    content: String,
}

// Define your API client with minimal configuration
beckon!(
    ApiClient,
    {
        {
            path: "/users",
            method: GET,
            res: Vec<User>,
        },
        {
            path: "/posts",
            method: GET,
            res: Post,
        },
    }
);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = Url::parse("https://api.example.com")?;
    // `timeout` takes a `Duration` (or `None` for the 5s default) — no ambiguous
    // millisecond integers.
    let client = ApiClient::new(base_url, Duration::from_secs(5));

    // Want to configure the underlying HTTP client — a shared connection pool,
    // TLS, proxy, or default headers? Build your own `reqwest::Client` and pass
    // it in with `with_client` instead (see `advanced.rs`):
    //
    //     let http = reqwest::Client::builder().user_agent("my-app/1.0").build()?;
    //     let client = ApiClient::with_client(base_url, http, Duration::from_secs(5));

    // Use the auto-generated methods
    let users = client.get_users().await?;
    println!("Found {} users", users.len());
    if let Some(user) = users.first() {
        println!("First user: #{} - {} ({})", user.id, user.name, user.email);
    }

    let post = client.get_posts().await?;
    println!("Post #{}: {} - {}", post.id, post.title, post.content);

    Ok(())
}
