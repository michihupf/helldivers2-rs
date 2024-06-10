use serde::Deserialize;

use crate::{
    prelude::{Parseable, Result},
    HellApi,
};

use super::planet::Planet;

/// Represents an ongoing campaign on a planet.
#[derive(Debug, Deserialize)]
pub struct Campaign {
    /// The unique identifier of this campaign.
    pub id: i32,
    /// The planet on which the campaign takes place.
    pub planet: Planet,
    /// The type of campaign. TODO This should be mapped onto an enum.
    #[serde(rename = "type")]
    pub _type: i32,
    /// Indicates how many campaigns have already been fought on this planet.
    pub count: u64,
}

impl Parseable<Campaign> for Campaign {}
impl Parseable<Vec<Campaign>> for Vec<Campaign> {}

impl HellApi {
    /// Retrieves a list of all available campaign information.
    ///
    /// Endpoint: `/api/v1/campaigns`.
    pub async fn campaigns(&self) -> Result<Vec<Campaign>> {
        self.request_blocking("/api/v1/campaigns").await
    }

    /// Retrieves a specific campaign with identifier `id`.
    ///
    /// Endpoint: `/api/v1/campaigns/{id}`.
    pub async fn campaign(&self, id: i32) -> Result<Campaign> {
        let endpoint = format!("/api/v1/campaigns/{id}");
        self.request_blocking(endpoint.as_str()).await
    }
}

#[cfg(test)]
mod testing {
    use crate::prelude::testing::HELL_API_TEST;

    #[tokio::test]
    async fn campaigns_endpoint() {
        let result = HELL_API_TEST.campaigns().await;
        let inner = result.unwrap();
        let first = inner.first().unwrap();

        let result = HELL_API_TEST.campaign(first.id).await;
        result.unwrap();
    }
}
