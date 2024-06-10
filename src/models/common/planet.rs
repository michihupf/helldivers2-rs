use serde::Deserialize;

use super::assignment::{CampaignId, JointOperationId};

/// Represents an ongoing event on a planet.
#[derive(Debug, Deserialize)]
pub struct Event {
    /// The unique identifier of the event.
    id: i32,
    /// The index of the affected planet.
    #[serde(rename = "planetIndex")]
    planet_index: i32,
    /// The identifier indicating the type of the event.
    #[serde(rename = "eventType")]
    event_type: i32, // TODO enum type PlanetEventType
    /// The faction identifier that owns the planet.
    race: i32, // TODO enum type Race
    /// The current health of the event.
    health: i64,
    /// The maximum health of the event.
    #[serde(rename = "maxHealth")]
    max_health: i64,
    /// The time at which this event starts.
    #[serde(rename = "startTime")]
    start: u64,
    /// The time at which this event ends.
    #[serde(rename = "expireTime")]
    expire: u64,
    /// The identifier of a related Campagin.
    #[serde(rename = "campaignId")]
    campaign_id: CampaignId,
    /// A list of identifiers for related joint operations.
    #[serde(rename = "jointOperationIds")]
    joint_operations: Vec<JointOperationId>,
}

/// Represents the coordinates returned by the ArrowHead API.
#[derive(Debug, Deserialize)]
pub struct Position {
    x: f32,
    y: f32,
}
