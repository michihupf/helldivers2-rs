use chrono::NaiveDateTime;
use proc::parse_test;
use serde::Deserialize;
use serde_with::TimestampSeconds;

use crate::models::common::{
    self,
    assignment::{CampaignId, JointOperationId},
};

/// Represents information about the homeworld(s) of a given race (faction).
#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct HomeWorld {
    pub race: i32,
    /// A list of planet index identifiers.
    #[serde(rename = "planetIndices")]
    pub planet_indices: Vec<i32>,
}

/// Represents the current status of a planet in the war.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
#[parse_test(make_parseable)]
pub struct PlanetStatus {
    /// The identifier of the corresponding PlanetInfo.
    pub index: i32,
    /// The faction currently owning the planet.
    pub owner: i32,
    /// The current liberation of a planet.
    pub health: i64,
    /// The health regeneration if the planet was left alone.
    #[serde(rename = "regenPerSecond")]
    pub regen_per_sec: f64,
    /// The amount of players active on this planet.
    pub players: u64,
    // TODO: position field
}

/// Represents an attack on a planet.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
#[parse_test(make_parseable)]
pub struct PlanetAttack {
    /// The identifier of where the attack originates from.
    pub source: i32,
    /// The planet that is targeted by the attack.
    pub target: i32,
}

/// Represents an ongoing event on a planet.
#[non_exhaustive]
#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
#[parse_test(make_parseable)]
pub struct PlanetEvent {
    /// The unique identifier of the event.
    pub id: i32,
    /// The index of the affected planet.
    #[serde(rename = "planetIndex")]
    pub planet_index: i32,
    /// The identifier indicating the type of the event.
    #[serde(rename = "eventType")]
    pub event_type: i32, // TODO enum type PlanetEventType
    /// The faction identifier that owns the planet.
    pub race: i32, // TODO enum type Race
    /// The current health of the event.
    pub health: i64,
    /// The maximum health of the event.
    #[serde(rename = "maxHealth")]
    pub max_health: i64,
    /// The time at which this event starts.
    #[serde(rename = "startTime")]
    #[serde_as(as = "TimestampSeconds")]
    pub start: NaiveDateTime,
    /// The time at which this event ends.
    #[serde(rename = "expireTime")]
    #[serde_as(as = "TimestampSeconds")]
    pub expire: NaiveDateTime,
    /// The identifier of a related Campagin.
    #[serde(rename = "campaignId")]
    pub campaign_id: CampaignId,
    /// A list of identifiers for related joint operations.
    #[serde(rename = "jointOperationIds")]
    pub joint_operations: Vec<JointOperationId>,
}

/// Represents the coordinates returned by the ArrowHead API.
pub type PlanetCoordinates = common::planet::Position;

/// Represents information of a planet.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
#[parse_test(make_parseable)]
pub struct PlanetInfo {
    /// The identifier for the planet.
    pub index: i32,
    /// Purpose is unknown at this point.
    #[serde(rename = "settingsHash")]
    pub settings_hash: i64,
    /// A set of X/Y coordinates specifying the position of the planet.
    pub position: PlanetCoordinates,
    /// A list of links to other planets (supply lines).
    pub waypoints: Vec<i32>,
    /// The identifier of the sector the planet is located in.
    pub sector: i32,
    /// The health of the planet.
    #[serde(rename = "maxHealth")]
    pub max_health: i64,
    /// Whether this planet is currently active in the galactic war.
    pub disabled: bool,
    /// The identifier of the faction that initially owned the planet.
    #[serde(rename = "initialOwner")]
    pub initial_owner: i32,
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;

    use crate::{
        models::{
            common::planet::Position,
            raw::planet::{PlanetAttack, PlanetEvent, PlanetInfo, PlanetStatus},
        },
        prelude::TestValue,
    };

    impl TestValue for PlanetStatus {
        const TEST_JSON: &'static str = r#"
            {
              "index": 0,
              "owner": 1,
              "health": 1000000,
              "regenPerSecond": 4.1666665,
              "players": 147,
              "position": {
                "x": 0,
                "y": 0
              }
            }
        "#;

        fn test_expected() -> Self {
            PlanetStatus {
                index: 0,
                owner: 1,
                health: 1000000,
                regen_per_sec: 4.1666665,
                players: 147,
            }
        }
    }

    impl TestValue for PlanetAttack {
        const TEST_JSON: &'static str = r#"
            {
              "source": 219,
              "target": 175
            }
        "#;

        fn test_expected() -> Self {
            PlanetAttack {
                source: 219,
                target: 175,
            }
        }
    }

    impl TestValue for PlanetEvent {
        const TEST_JSON: &'static str = r#"{
          "id": 0,
          "planetIndex": 1,
          "eventType": 2,
          "race": 3,
          "health": 4,
          "maxHealth": 5,
          "startTime": 64880360,
          "expireTime": 65303717,
          "campaignId": 8,
          "jointOperationIds": []
        }"#;

        fn test_expected() -> Self {
            PlanetEvent {
                id: 0,
                planet_index: 1,
                event_type: 2,
                race: 3,
                health: 4,
                max_health: 5,
                start: DateTime::from_timestamp(64880360, 0).unwrap().naive_utc(),
                expire: DateTime::from_timestamp(65303717, 0).unwrap().naive_utc(),
                campaign_id: 8,
                joint_operations: vec![],
            }
        }
    }

    impl TestValue for PlanetInfo {
        const TEST_JSON: &'static str = r#"{
          "index": 1,
          "settingsHash": 3621417917,
          "planetNameId32": 0,
          "position": {
            "x": 0.05373042,
            "y": 0.10565466
          },
          "waypoints": [],
          "sector": 1,
          "maxHealth": 1000000,
          "disabled": false,
          "initialOwner": 1
        }"#;

        fn test_expected() -> Self {
            PlanetInfo {
                index: 1,
                settings_hash: 3621417917,
                position: Position {
                    x: 0.05373042,
                    y: 0.10565466,
                },
                waypoints: vec![],
                sector: 1,
                max_health: 1000000,
                disabled: false,
                initial_owner: 1,
            }
        }
    }
}
