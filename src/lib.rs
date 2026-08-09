#![doc = include_str!("../README.md")]

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
