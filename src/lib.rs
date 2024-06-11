#![allow(dead_code)]

pub mod middleware;
pub mod models;
pub mod prelude;

/// Wrapper for the Helldivers 2 community driven API. This wrapper is blocking to ensure rate-limits.
/// Use this struct to query for information.
pub struct HellApi;
