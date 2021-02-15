#![allow(dead_code, unused_variables, unused_imports)]

#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "example")]
pub mod example;
pub mod models;
pub mod openapi_serialization;
#[cfg(feature = "server")]
pub mod security;

pub use models::*;

pub const VERSION: &str = "1.0.0";
