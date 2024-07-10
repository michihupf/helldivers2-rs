use lazy_static::lazy_static;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::middleware::RateLimit;

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

/// Helper struct to use newtype pattern.
pub(crate) struct W<T>(pub T);

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

// impl<T> TryFrom<W<serde_json::Value>> for T
// where
//     T: Parseable + DeserializeOwned,
// {
//     type Error = crate::prelude::Error;

//     fn try_from(value: W<serde_json::Value>) -> std::prelude::v1::Result<Self, Self::Error> {
//         Self::parse(value.0)
//     }
// }

/// Defines interface for testable values.
#[cfg(test)]
pub(crate) trait TestValue {
    /// Returns the expected result corresponding to the [`test_json`][test_json].
    fn test_expected() -> Self;

    /// Returns the raw json string for a test.
    const TEST_JSON: &'static str;
}

lazy_static! {
    pub(crate) static ref RATE_LIMIT: RateLimit = RateLimit::default();
}
