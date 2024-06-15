use std::time::Duration;

use lazy_static::lazy_static;
use ratelimit::Ratelimiter;
use serde::de::DeserializeOwned;
use serde_json::Value;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // #[error("ERROR")]
    // General,
    /// HTTP request failed to complete successfully.
    #[error("HTTP request failed. {0:?}")]
    RequestError(#[from] reqwest::Error),
    /// The Rate-Limit has been reached.
    #[error("Rate limit reached.")]
    RateLimitReached(core::time::Duration),
    /// Parsing of JSON response failed.
    #[error("Parsing of JSON failed. {0}")]
    ParseError(#[from] serde_json::Error),
}

pub type Result<T> = core::result::Result<T, Error>;

/// Specifies that the object is JSON parseable and provides
/// the `parse()` method to return a struct of type `T`.
pub(crate) trait Parseable {
    /// Parses the JSON response from the API endpoint and returns `Ok(T)`
    /// if parsing succeeded - `Err(HellHubError)` otherwise.
    fn parse(json: Value) -> Result<Self>
    where
        Self: DeserializeOwned,
    {
        serde_json::from_value::<Self>(json).map_err(Error::ParseError)
    }
}

lazy_static! {
    pub static ref RATE_LIMITER: Ratelimiter = Ratelimiter::builder(5, Duration::from_secs(11))
        .max_tokens(5)
        .build()
        .unwrap();
}
