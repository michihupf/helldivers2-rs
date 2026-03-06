use chrono::NaiveDateTime;
use proc::{parse_test, Parseable};
use serde::Deserialize;
use serde_with::TimestampSeconds;

use crate::{middleware, prelude::Result, HellApi};

use super::{
    campaign::Campaign,
    planet::{HomeWorld, PlanetAttack, PlanetEvent, PlanetInfo, PlanetStatus},
    stats::{GalaxyStats, PlanetStats},
};

/// Type of the ID returned from the WarID endpoint.
#[non_exhaustive]
#[repr(transparent)]
#[derive(Debug, Deserialize, Parseable)]
pub struct WarId {
    pub id: i32,
}

impl From<i32> for WarId {
    fn from(value: i32) -> Self {
        Self { id: value }
    }
}

/// Represents a snapshot of the current status of the
/// galactic war.
#[non_exhaustive]
#[derive(Debug, Deserialize, Parseable)]
#[parse_test]
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

/// Represents information about the current war.
#[non_exhaustive]
#[serde_with::serde_as]
#[derive(Debug, Deserialize, Parseable)]
// #[parse_test]
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

/// Represents general statistics about the galaxy and specific planets.
#[non_exhaustive]
#[derive(Debug, Deserialize, Parseable)]
// #[parse_test]
pub struct WarSummary {
    /// Galaxy wide statistics aggregated from all planets.
    pub galaxy_stats: GalaxyStats,
    // /// Statistics for specific planets.
    #[serde(rename = "planets_stats")]
    pub planet_stats: Vec<PlanetStats>,
}

/// Represents a joint operation.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
// #[parse_test(make_parseable)]
pub struct JointOperation {
    pub id: i32,
    #[serde(rename = "planetIndex")]
    pub planet_index: i32,
    #[serde(rename = "hqNodeIndex")]
    pub hq_node_index: i32,
}

impl HellApi {
    impl_route!(
        "/raw/api/WarSeason/current/WarID",
        war_id,
        WarId,
        "Requests the current war id."
    );

    /// Requests the current war status.
    ///
    /// Endpoint: `/raw/api/WarSeason/{war_id}/Status`.
    pub async fn war_status(war_id: &WarId) -> Result<WarStatus> {
        let endpoint = format!("/raw/api/WarSeason/{}/Status", war_id.id);
        middleware::request_blocking(endpoint.as_str()).await
    }

    /// Requests the current war info.
    ///
    /// Endpoint: `/raw/api/WarSeason/{war_id}/WarInfo`.
    pub async fn war_info(war_id: &WarId) -> Result<WarInfo> {
        let endpoint = format!("/raw/api/WarSeason/{}/WarInfo", war_id.id);
        middleware::request_blocking(endpoint.as_str()).await
    }

    /// Requests the current war summary.
    ///
    /// Endpoint: `/raw/api/Stats/war/{war_id}/summary`.
    pub async fn war_summary(war_id: &WarId) -> Result<WarSummary> {
        let endpoint = format!("/raw/api/Stats/war/{}/summary", war_id.id);
        middleware::request_blocking(endpoint.as_str()).await
    }
}

#[cfg(test)]
mod tests {

    use crate::{models::raw::war::WarStatus, prelude::TestValue};

    impl TestValue for WarStatus {
        const TEST_JSON: &'static str = r#"{
              "warId": 801,
              "time": 64880360,
              "impactMultiplier": 0.018466096,
              "storyBeatId32": 0,
              "planetStatus": [],
              "planetAttacks": [],
              "campaigns": [],
              "communityTargets": [],
              "jointOperations": [],
              "planetEvents": [],
              "planetActiveEffects": [
                {
                  "index": 256,
                  "galacticEffectId": 1190
                }
              ],
              "planetRegions": [
                {
                  "planetIndex": 114,
                  "regionIndex": 0,
                  "owner": 3,
                  "health": 400000,
                  "regerPerSecond": 1.1111112,
                  "availabilityFactor": 0.57685,
                  "isAvailable": false,
                  "players": 0
                }
              ],
              "activeElectionPolicyEffects": [],
              "globalEvents": [
                {
                  "eventId": 1500817,
                  "id32": 2520682855,
                  "portraitId32": 0,
                  "title": "BRIEFING",
                  "titleId32": 2908633975,
                  "message": "Classified Ministry of Science readouts indicate a severe escalation of...",
                  "messageId32": 2649354563,
                  "race": 1,
                  "flag": 1,
                  "introMediaId32": 2035168874,
                  "outroMediaId32": 0,
                  "assignmentId32": 77022345,
                  "effectIds": [],
                  "planetIndices": [],
                  "expireTime": 65303717
                }
              ],
              "superEarthWarResults": [],
              "spaceStations": [
                {
                  "id32": 749875195,
                  "planetIndex": 180,
                  "activeEffectIds": [
                    1238,
                    1261,
                    1262
                  ],
                  "currentElectionEndWarTime": 64886460,
                  "flags": 1
                }
              ],
              "globalResources": [],
              "layoutVersion": 479
            }"#;

        fn test_expected() -> Self {
            WarStatus {
                war_id: 801,
                time: 64880360,
                impact_multiplier: 0.018466096,
                story_beat_id32: 0,
                planet_status: vec![],
                planet_attacks: vec![],
                campaigns: vec![],
                joint_operations: vec![],
                planet_events: vec![],
            }
        }
    }
}
