//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;

use calculator::krypto::{setup, valid_input};
/// Echo the user input on the server.
#[post("/api/echo")]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

