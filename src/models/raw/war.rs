use chrono::NaiveDateTime;
use serde::Deserialize;
use serde_with::TimestampSeconds;

use crate::{
    prelude::{Parseable, Result},
    HellApi,
};

use super::{
    assignment::{Campaign, JointOperation},
    planet::{HomeWorld, PlanetAttack, PlanetEvent, PlanetInfo, PlanetStatus},
    stats::{GalaxyStats, PlanetStats},
};

/// Type of the ID returned from the WarID endpoint.
#[repr(transparent)]
#[derive(Debug, Deserialize)]
pub struct WarId {
    pub id: i32,
}

impl From<i32> for WarId {
    fn from(value: i32) -> Self {
        Self { id: value }
    }
}

impl Parseable<WarId> for WarId {}

/// Represents a snapshot of the current status of the
/// galactic war.
#[derive(Debug, Deserialize)]
pub struct WarStatus {
    /// The war season this status refers to.
    #[serde(rename = "warId")]
    war_id: i32,
    /// The time the snapshot was taken.
    time: u64,
    /// The factor by which influence at mission end is
    /// multiplied to calculate the mission impact on
    /// liberation.
    #[serde(rename = "impactMultiplier")]
    impact_multiplier: f32,
    /// Internal identifier. Purpose unknown.
    #[serde(rename = "storyBeatId32")]
    story_beat_id32: i64,
    /// A list of planet statuses.
    #[serde(rename = "planetStatus")]
    planet_status: Vec<PlanetStatus>,
    /// A list of current planet attacks.
    #[serde(rename = "planetAttacks")]
    planet_attacks: Vec<PlanetAttack>,
    /// A list of ongoing campaigns in the war.
    campaigns: Vec<Campaign>,
    /// A list of JointOperations.
    #[serde(rename = "jointOperations")]
    joint_operations: Vec<JointOperation>,
    /// A list of current planet events.
    #[serde(rename = "planetEvents")]
    planet_events: Vec<PlanetEvent>,
}

impl Parseable<WarStatus> for WarStatus {}

/// Represents information about the current war.
#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
pub struct WarInfo {
    /// The war season this WarInfo refers to.
    #[serde(rename = "warId")]
    war_id: i32,
    /// The start time of the season.
    #[serde(rename = "startDate")]
    #[serde_as(as = "TimestampSeconds")]
    start: NaiveDateTime,
    /// The end time of the season.
    #[serde(rename = "endDate")]
    #[serde_as(as = "TimestampSeconds")]
    end: NaiveDateTime,
    /// A version string that indicates the minimum game client version
    /// the API supports.
    #[serde(rename = "minimumClientVersion")]
    minimum_client_version: String,
    /// A list of planets that are involved in the war.
    #[serde(rename = "planetInfos")]
    planet_infos: Vec<PlanetInfo>,
    /// A list of homeworlds for the races (factions) invld in the war.
    #[serde(rename = "homeWorlds")]
    home_worlds: Vec<HomeWorld>,
}

impl Parseable<WarInfo> for WarInfo {}

/// Represents general statistics about the galaxy and specific planets.
#[derive(Debug, Deserialize)]
pub struct WarSummary {
    /// Galaxy wide statistics aggregated from all planets.
    galaxy_stats: GalaxyStats,
    // /// Statistics for specific planets.
    #[serde(rename = "planets_stats")]
    planet_stats: Vec<PlanetStats>,
}

impl Parseable<WarSummary> for WarSummary {}

impl HellApi {
    /// Requests the current war id.
    ///
    /// Endpoint: `/raw/api/WarSeason/current/WarID`.
    pub async fn war_id(&self) -> Result<WarId> {
        self.request_blocking("/raw/api/WarSeason/current/WarID")
            .await
    }

    /// Requests the current war status.
    ///
    /// Endpoint: `/raw/api/WarSeason/{war_id}/Status`.
    pub async fn war_status(&self, war_id: &WarId) -> Result<WarStatus> {
        let endpoint = format!("/raw/api/WarSeason/{}/Status", war_id.id);
        self.request_blocking(endpoint.as_str()).await
    }

    /// Requests the current war info.
    ///
    /// Endpoint: `/raw/api/WarSeason/{war_id}/WarInfo`.
    pub async fn war_info(&self, war_id: &WarId) -> Result<WarInfo> {
        let endpoint = format!("/raw/api/WarSeason/{}/WarInfo", war_id.id);
        self.request_blocking(endpoint.as_str()).await
    }

    /// Requests the current war summary.
    ///
    /// Endpoint: `/raw/api/Stats/war/{war_id}/summary`.
    pub async fn war_summary(&self, war_id: &WarId) -> Result<WarSummary> {
        let endpoint = format!("/raw/api/Stats/war/{}/summary", war_id.id);
        self.request_blocking(endpoint.as_str()).await
    }
}

#[cfg(test)]
mod testing {
    use crate::{models::raw::war::WarId, prelude::testing::HELL_API_TEST};

    #[tokio::test]
    async fn war_id_endpoint() {
        let result = HELL_API_TEST.war_id().await;
        assert!(result.is_ok(), "{result:?}");
    }

    #[tokio::test]
    async fn war_status_endpoint() {
        let result = HELL_API_TEST.war_status(&WarId::from(801)).await;
        assert!(result.is_ok(), "{result:?}");
    }

    #[tokio::test]
    async fn war_info_endpoint() {
        let result = HELL_API_TEST.war_info(&WarId::from(801)).await;
        assert!(result.is_ok(), "{result:?}");
    }

    #[tokio::test]
    async fn war_summary_endpoint() {
        let result = HELL_API_TEST.war_summary(&WarId::from(801)).await;
        assert!(result.is_ok(), "{result:?}");
    }
}
