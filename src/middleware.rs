//! This middleware module enforces rate limits on requests to ensure
//! the fair use limitations of the HellHub API.

use serde::de::DeserializeOwned;

use crate::{
    prelude::{Error, Parseable, Result},
    HellApi,
};

/// helldivers2 API base url
const BASE_URL: &str = "https://api.helldivers2.dev";

impl HellApi {
    pub(crate) async fn request<T>(&self, endpoint: &str) -> Result<T>
    where
        T: DeserializeOwned + Parseable<T>,
    {
        self.rate_limiter
            .try_wait()
            .map_err(Error::RateLimitReached)?;

        let response = reqwest::get(BASE_URL.to_owned() + endpoint).await?;

        T::parse(response).await
    }

    /// Requests the API `endpoint` blocking the current thread when the `rate` limit has been reached.
    /// Afterwards the JSON response is deserialized into `T`.
    pub(crate) async fn request_blocking<T>(&self, endpoint: &str) -> Result<T>
    where
        T: DeserializeOwned + Parseable<T>,
    {
        // block until ready
        while let Err(wait_for) = self.rate_limiter.try_wait() {
            tokio::time::sleep(wait_for).await;
        }

        let response = reqwest::get(BASE_URL.to_owned() + endpoint).await?;

        T::parse(response).await
    }
}
