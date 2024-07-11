//! This middleware module enforces rate limits on requests to ensure
//! the fair use limitations of the HellHub API.

use std::{
    sync::atomic::{AtomicU32, AtomicU64, Ordering},
    time::Duration,
};

use reqwest::header::HeaderMap;

use crate::prelude::{Error, Parseable, Result, RATE_LIMIT};

/// helldivers2 API base url
const BASE_URL: &str = "https://api.helldivers2.dev";

pub(crate) struct RateLimit {
    /// Defines the request limit.
    x_limit: AtomicU32,
    /// How many requests are still remaining.
    x_remaining: AtomicU32,
    /// Retry delay in seconds when received a HTTP429. The value `0` will be treated as a None value.
    retry_after: AtomicU64,
}

impl RateLimit {
    /// Updates the rate-limit information using the latest response headers.
    fn update(&self, headers: &HeaderMap) {
        let limit = headers
            .get("X-Ratelimit-Limit")
            .map(|v| v.to_str().unwrap_or("0").parse::<u32>().unwrap_or(0u32))
            .unwrap_or(0);
        let remaining = headers
            .get("X-Ratelimit-Remaining")
            .map(|v| v.to_str().unwrap_or("0").parse::<u32>().unwrap_or(0u32))
            .unwrap_or(0);
        let retry_after = headers
            .get("Retry-After")
            .map(|v| v.to_str().unwrap_or("0").parse::<u64>().unwrap_or(1u64));

        self.x_limit.store(limit, Ordering::Relaxed);
        self.x_remaining.store(remaining, Ordering::Relaxed);
        self.retry_after
            .store(retry_after.unwrap_or(0), Ordering::Relaxed);
    }

    /// Returns whether a request is allowed.
    fn allowed(&self) -> bool {
        self.x_remaining.load(Ordering::Relaxed) > 0 && self.retry_after.load(Ordering::Relaxed) < 1
    }

    /// Returns the `Ok` variant if a request is allowed. Otherwise returns the amount of time to wait
    /// until retrying.
    fn try_wait(&self) -> core::result::Result<(), Duration> {
        if self.allowed() {
            Ok(())
        } else {
            let retry = self.retry_after.load(Ordering::Relaxed);
            Err(Duration::from_secs(match retry {
                0 => 1,
                v => v + 1,
            }))
        }
    }
}

impl Default for RateLimit {
    fn default() -> Self {
        RateLimit {
            x_limit: AtomicU32::new(5),
            x_remaining: AtomicU32::new(0),
            retry_after: AtomicU64::new(0),
        }
    }
}

#[allow(dead_code)]
pub(crate) async fn request<T: Parseable>(endpoint: &str) -> Result<T> {
    if let Err(duration) = RATE_LIMIT.try_wait() {
        return Err(Error::RateLimitReached(duration));
    }

    let response = reqwest::get(BASE_URL.to_owned() + endpoint).await?;
    RATE_LIMIT.update(response.headers());

    let json: serde_json::Value = response.json().await.map_err(Error::RequestError)?;

    T::parse(json)
}

/// Requests the API `endpoint` blocking the current thread when the `rate` limit has been reached.
/// Afterwards the JSON response is deserialized into `T`.
pub(crate) async fn request_blocking<T: Parseable>(endpoint: &str) -> Result<T> {
    // block until ready
    let response = loop {
        if let Err(wait_for) = RATE_LIMIT.try_wait() {
            tokio::time::sleep(wait_for).await;
            if let Ok(response) = reqwest::get(BASE_URL.to_owned() + endpoint).await {
                if response.status() == 200 {
                    break response;
                } else {
                    RATE_LIMIT.update(response.headers());
                }
            };
        } else {
            break reqwest::get(BASE_URL.to_owned() + endpoint).await?;
        }
    };

    RATE_LIMIT.update(response.headers());
    let json = response.json().await.map_err(Error::RequestError)?;

    T::parse(json)
}
