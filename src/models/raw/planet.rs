use chrono::NaiveDateTime;
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
}

/// Represents an attack on a planet.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
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
