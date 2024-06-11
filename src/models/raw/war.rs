use chrono::NaiveDateTime;
use serde::Deserialize;
use serde_with::TimestampSeconds;

use crate::{
    prelude::{Parseable, Result},
    HellApi,
};

use super::{
    campaign::Campaign,
    planet::{HomeWorld, PlanetAttack, PlanetEvent, PlanetInfo, PlanetStatus},
    stats::{GalaxyStats, PlanetStats},
};

/// Type of the ID returned from the WarID endpoint.
#[non_exhaustive]
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
#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct WarStatus {
    /// The war season this status refers to.
    #[serde(rename = "warId")]
    pub war_id: i32,
    /// The time the snapshot was taken.
    pub time: u64,
    /// The factor by which influence at mission end is
    /// multiplied to calculate the mission impact on
    /// liberation.
    #[serde(rename = "impactMultiplier")]
    pub impact_multiplier: f32,
    /// Internal identifier. Purpose unknown.
    #[serde(rename = "storyBeatId32")]
    pub story_beat_id32: i64,
    /// A list of planet statuses.
    #[serde(rename = "planetStatus")]
    pub planet_status: Vec<PlanetStatus>,
    /// A list of current planet attacks.
    #[serde(rename = "planetAttacks")]
    pub planet_attacks: Vec<PlanetAttack>,
    /// A list of ongoing campaigns in the war.
    pub campaigns: Vec<Campaign>,
    /// A list of JointOperations.
    #[serde(rename = "jointOperations")]
    pub joint_operations: Vec<JointOperation>,
    /// A list of current planet events.
    #[serde(rename = "planetEvents")]
    pub planet_events: Vec<PlanetEvent>,
}

impl Parseable<WarStatus> for WarStatus {}

/// Represents information about the current war.
#[non_exhaustive]
#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
pub struct WarInfo {
    /// The war season this WarInfo refers to.
    #[serde(rename = "warId")]
    pub war_id: i32,
    /// The start time of the season.
    #[serde(rename = "startDate")]
    #[serde_as(as = "TimestampSeconds")]
    pub start: NaiveDateTime,
    /// The end time of the season.
    #[serde(rename = "endDate")]
    #[serde_as(as = "TimestampSeconds")]
    pub end: NaiveDateTime,
    /// A version string that indicates the minimum game client version
    /// the API supports.
    #[serde(rename = "minimumClientVersion")]
    pub minimum_client_version: String,
    /// A list of planets that are involved in the war.
    #[serde(rename = "planetInfos")]
    pub planet_infos: Vec<PlanetInfo>,
    /// A list of homeworlds for the races (factions) invld in the war.
    #[serde(rename = "homeWorlds")]
    pub home_worlds: Vec<HomeWorld>,
}

impl Parseable<WarInfo> for WarInfo {}

/// Represents general statistics about the galaxy and specific planets.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct WarSummary {
    /// Galaxy wide statistics aggregated from all planets.
    pub galaxy_stats: GalaxyStats,
    // /// Statistics for specific planets.
    #[serde(rename = "planets_stats")]
    pub planet_stats: Vec<PlanetStats>,
}

impl Parseable<WarSummary> for WarSummary {}

/// Represents a joint operation.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct JointOperation {
    pub id: i32,
    #[serde(rename = "planetIndex")]
    pub planet_index: i32,
    #[serde(rename = "hqNodeIndex")]
    pub hq_node_index: i32,
}

impl HellApi {
    /// Requests the current war id.
    ///
    /// Endpoint: `/raw/api/WarSeason/current/WarID`.
    pub async fn war_id() -> Result<WarId> {
        Self::request_blocking("/raw/api/WarSeason/current/WarID").await
    }

    /// Requests the current war status.
    ///
    /// Endpoint: `/raw/api/WarSeason/{war_id}/Status`.
    pub async fn war_status(war_id: &WarId) -> Result<WarStatus> {
        let endpoint = format!("/raw/api/WarSeason/{}/Status", war_id.id);
        Self::request_blocking(endpoint.as_str()).await
    }

    /// Requests the current war info.
    ///
    /// Endpoint: `/raw/api/WarSeason/{war_id}/WarInfo`.
    pub async fn war_info(war_id: &WarId) -> Result<WarInfo> {
        let endpoint = format!("/raw/api/WarSeason/{}/WarInfo", war_id.id);
        Self::request_blocking(endpoint.as_str()).await
    }

    /// Requests the current war summary.
    ///
    /// Endpoint: `/raw/api/Stats/war/{war_id}/summary`.
    pub async fn war_summary(war_id: &WarId) -> Result<WarSummary> {
        let endpoint = format!("/raw/api/Stats/war/{}/summary", war_id.id);
        Self::request_blocking(endpoint.as_str()).await
    }
}
