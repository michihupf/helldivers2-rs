use serde::Deserialize;

use super::assignment::{CampaignId, JointOperationId};

/// Represents an ongoing event on a planet.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Event {
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
    pub start: u64,
    /// The time at which this event ends.
    #[serde(rename = "expireTime")]
    pub expire: u64,
    /// The identifier of a related Campagin.
    #[serde(rename = "campaignId")]
    pub campaign_id: CampaignId,
    /// A list of identifiers for related joint operations.
    #[serde(rename = "jointOperationIds")]
    pub joint_operations: Vec<JointOperationId>,
}

/// Represents the coordinates returned by the ArrowHead API.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
