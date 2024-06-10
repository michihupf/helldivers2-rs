#![allow(dead_code)]

pub mod middleware;
pub mod models;
pub mod prelude;

use std::time::Duration;

use ratelimit::Ratelimiter;

/// Wrapper for the Helldivers 2 community driven API. This wrapper is blocking to ensure rate-limits.
/// Use this struct to query for information.
pub struct HellApi {
    rate_limiter: Ratelimiter,
}

impl HellApi {
    pub fn new() -> HellApi {
        HellApi {
            rate_limiter: Ratelimiter::builder(5, Duration::from_secs(11)) // wait an extra second to ensure rate limits
                .max_tokens(5)
                .build()
                .unwrap(),
        }
    }
}

impl Default for HellApi {
    fn default() -> Self {
        Self::new()
    }
}
