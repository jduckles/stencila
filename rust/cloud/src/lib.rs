use std::env;

use common::{once_cell::sync::Lazy, serde::Deserialize};

/// The base URL for the Stencila Cloud API
///
/// Can be overridden by setting the STENCILA_API_URL environment variable.
const BASE_URL: &str = "https://api.stencila.cloud/v1";

/// Get the base URL for the Stencila Cloud API
pub fn base_url() -> String {
    env::var("STENCILA_API_URL").unwrap_or_else(|_| BASE_URL.to_string())
}

/// The name of the env var or secret for the API key
const API_KEY_NAME: &str = "STENCILA_API_TOKEN";

/// The API key value. Stored to avoid repeated access to secrets (which is
/// relatively slow) for each model
static API_KEY: Lazy<Option<String>> = Lazy::new(|| secrets::env_or_get(API_KEY_NAME).ok());

/// Get the API key for the Stencila Cloud API
pub fn api_key() -> &'static Option<String> {
    &API_KEY
}

/// An error response from Stencila Cloud
#[derive(Default, Deserialize)]
#[serde(default, crate = "common::serde")]
pub struct ErrorResponse {
    pub status: u16,
    pub error: String,
}