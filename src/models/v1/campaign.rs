use serde::Deserialize;

use crate::{
    prelude::{Parseable, Result},
    HellApi,
};

use super::planet::Planet;

/// Represents an ongoing campaign on a planet.
#[non_exhaustive]
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
    pub async fn campaigns() -> Result<Vec<Campaign>> {
        Self::request_blocking("/api/v1/campaigns").await
    }

    /// Retrieves a specific campaign with identifier `id`.
    ///
    /// Endpoint: `/api/v1/campaigns/{id}`.
    pub async fn campaign(id: i32) -> Result<Campaign> {
        let endpoint = format!("/api/v1/campaigns/{id}");
        Self::request_blocking(endpoint.as_str()).await
    }
}
