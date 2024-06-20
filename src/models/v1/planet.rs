use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;

use crate::{
    middleware,
    models::common::{
        self,
        assignment::{CampaignId, JointOperationId},
    },
    prelude::{Parseable, Result},
    HellApi,
};

use super::{dispatch::Message, stats::Statistics};

/// Represents the coordinates returned by the ArrowHead API.
pub type Position = common::planet::Position;

/// Represents an ongoing event on a planet.
#[non_exhaustive]
#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
pub struct Event {
    /// The unique identifier of the event.
    pub id: i32,
    /// The identifier indicating the type of the event.
    #[serde(rename = "eventType")]
    pub event_type: i32, // TODO enum type PlanetEventType
    /// The faction identifier that owns the planet.
    pub faction: String,
    /// The current health of the event.
    pub health: i64,
    /// The maximum health of the event.
    #[serde(rename = "maxHealth")]
    pub max_health: i64,
    /// The time at which this event starts.
    #[serde(rename = "startTime")]
    #[serde_as(as = "DateTime<Utc>")]
    pub start: NaiveDateTime,
    /// The time at which this event ends.
    #[serde(rename = "endTime")]
    #[serde_as(as = "DateTime<Utc>")]
    pub end: NaiveDateTime,
    /// The identifier of a related Campagin.
    #[serde(rename = "campaignId")]
    pub campaign_id: CampaignId,
    /// A list of identifiers for related joint operations.
    #[serde(rename = "jointOperationIds")]
    pub joint_operations: Vec<JointOperationId>,
}

/// Contains all aggregated information ArrowHead has about a planet.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Planet {
    /// The unique identifier ArrowHead assigned to this planet.
    #[serde(rename = "index")]
    pub id: i32,
    /// The name of the planet as shown in game.
    pub name: Message,
    /// The name of sector the planet is in as shown in game.
    pub sector: String,
    /// The biome the planet is in.
    pub biome: Biome,
    /// All hazards present on this planet.
    pub hazards: Vec<Hazard>,
    /// A hash assigned to the planet by ArrowHead. Purpose unknown.
    pub hash: i64,
    /// The coordinates of the planet on the galactic war map.
    pub position: Position,
    /// A list of planet indices that this planet is connected to.
    pub waypoints: Vec<i32>,
    /// The maximum health pool of the planet.
    #[serde(rename = "maxHealth")]
    pub max_health: i64,
    /// The current health of the planet.
    pub health: i64,
    /// Whether the planet is disabled.
    pub disabled: bool,
    /// The faction that originally owned the planet.
    #[serde(rename = "initialOwner")]
    pub initial_owner: String,
    /// The faction that currently owns the planet.
    #[serde(rename = "currentOwner")]
    pub current_owner: String,
    /// How much the planet regenerates health when left alone.
    #[serde(rename = "regenPerSecond")]
    pub regen_per_second: f32,
    /// Information on the active event on this planet.
    pub event: Option<Event>,
    /// A set of statistics for this planet.
    pub statistics: Statistics,
    /// A list of planets currently attacked by this planet.
    pub attacking: Vec<i32>,
}

impl Parseable for Planet {}
impl Parseable for Vec<Planet> {}

/// Represents information about a biome of a Planet.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Biome {
    // The name of the biome.
    pub name: String,
    /// A short human-readable description of the biome.
    pub description: String,
}

/// Represents an environmental hazard that can be present on a Planet.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Hazard {
    /// The name of the environmental hazard.
    pub name: String,
    /// The description of the environmental hazard.
    pub description: String,
}

impl HellApi {
    /// Retrieves a list of all available planet information.
    ///
    /// Endpoint: `/api/v1/planets`.
    pub async fn planets() -> Result<Vec<Planet>> {
        middleware::request_blocking("/api/v1/planets").await
    }

    /// Retrieves a specific planet with identifier `id`.
    ///
    /// Endpoint: `/api/v1/planets/{id}`.
    pub async fn planet(id: i32) -> Result<Planet> {
        let endpoint = format!("/api/v1/planets/{id}");
        middleware::request_blocking(endpoint.as_str()).await
    }

    /// Retrieves a list of all planets with an active event.
    ///
    /// Endpoint: `/api/v1/planet-events`.
    pub async fn planet_events() -> Result<Vec<Planet>> {
        middleware::request_blocking("/api/v1/planet-events").await
    }
}
