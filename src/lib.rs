//! Generate type-safe, async HTTP clients from endpoint definitions.
//!
//! `beckon!` takes a client name and a list of endpoints and expands to a struct
//! with one async method per endpoint, a matching trait for mocking, and a typed
//! error enum.
//!
//! ```
//! use beckon::beckon;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize)]
//! pub struct User {
//!     pub id: u32,
//!     pub name: String,
//! }
//!
//! #[derive(Serialize)]
//! pub struct UserPath {
//!     pub id: u32,
//! }
//!
//! beckon!(
//!     UserApi,
//!     {
//!         {
//!             path: "/users",
//!             method: GET,
//!             res: Vec<User>,
//!         },
//!         {
//!             path: "/users/{id}",
//!             method: GET,
//!             path_params: UserPath,
//!             res: User,
//!         }
//!     }
//! );
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = UserApi::new(reqwest::Url::parse("https://api.example.com")?, Some(30));
//! let _users = client.get_users().await?;
//! let _user = client.get_users_by_id(&UserPath { id: 1 }).await?;
//! # Ok(())
//! # }
//! # fn main() {}
//! ```

extern crate proc_macro;

use crate::expanders::ApiClientExpander;
use crate::input::ApiClientInput;
use syn::parse_macro_input;

mod error;
mod expanders;
mod input;

fn expand_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ApiClientInput);
    match ApiClientExpander::new(input).expand() {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

/// Generate a type-safe, async HTTP client from endpoint definitions.
///
/// See the [crate-level docs](crate) for the full endpoint grammar and features
/// (auth, retries, path/query params, headers, custom method names).
#[proc_macro]
pub fn beckon(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand_macro(input)
}
