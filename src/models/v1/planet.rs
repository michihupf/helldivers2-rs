use serde::Deserialize;

use crate::{
    models::common,
    prelude::{Parseable, Result},
    HellApi,
};

use super::{dispatch::Message, stats::Statistics};

/// Represents the coordinates returned by the ArrowHead API.
pub type Position = common::planet::Position;

/// Represents an ongoing event on a planet.
pub type Event = common::planet::Event;

/// Contains all aggregated information ArrowHead has about a planet.
#[derive(Debug, Deserialize)]
pub struct Planet {
    /// The unique identifier ArrowHead assigned to this planet.
    #[serde(rename = "index")]
    id: i32,
    /// The name of the planet as shown in game.
    name: Message,
    /// The name of sector the planet is in as shown in game.
    sector: String,
    /// The biome the planet is in.
    biome: Biome,
    /// All hazards present on this planet.
    hazards: Vec<Hazard>,
    /// A hash assigned to the planet by ArrowHead. Purpose unknown.
    hash: i64,
    /// The coordinates of the planet on the galactic war map.
    position: Position,
    /// A list of planet indices that this planet is connected to.
    waypoints: Vec<i32>,
    /// The maximum health pool of the planet.
    #[serde(rename = "maxHealth")]
    max_health: i64,
    /// The current health of the planet.
    health: i64,
    /// Whether the planet is disabled.
    disabled: bool,
    /// The faction that originally owned the planet.
    #[serde(rename = "initialOwner")]
    initial_owner: String,
    /// The faction that currently owns the planet.
    #[serde(rename = "currentOwner")]
    current_owner: String,
    /// How much the planet regenerates health when left alone.
    #[serde(rename = "regenPerSecond")]
    regen_per_second: f32,
    /// Information on the active event on this planet.
    event: Option<Event>,
    /// A set of statistics for this planet.
    statistics: Statistics,
    /// A list of planets currently attacked by this planet.
    attacking: Vec<i32>,
}

impl Parseable<Planet> for Planet {}
impl Parseable<Vec<Planet>> for Vec<Planet> {}

/// Represents information about a biome of a Planet.
#[derive(Debug, Deserialize)]
pub struct Biome {
    // The name of the biome.
    name: String,
    /// A short human-readable description of the biome.
    description: String,
}

/// Represents an environmental hazard that can be present on a Planet.
#[derive(Debug, Deserialize)]
pub struct Hazard {
    /// The name of the environmental hazard.
    name: String,
    /// The description of the environmental hazard.
    description: String,
}

impl HellApi {
    /// Retrieves a list of all available planet information.
    ///
    /// Endpoint: `/api/v1/planets`.
    pub async fn planets(&self) -> Result<Vec<Planet>> {
        self.request_blocking("/api/v1/planets").await
    }

    /// Retrieves a specific planet with identifier `id`.
    ///
    /// Endpoint: `/api/v1/planets/{id}`.
    pub async fn planet(&self, id: i32) -> Result<Planet> {
        let endpoint = format!("/api/v1/planets/{id}");
        self.request_blocking(endpoint.as_str()).await
    }

    /// Retrieves a list of all planets with an active event.
    ///
    /// Endpoint: `/api/v1/planet-events`.
    pub async fn planet_events(&self) -> Result<Vec<Planet>> {
        self.request_blocking("/api/v1/planet-events").await
    }
}

#[cfg(test)]
mod testing {
    use crate::prelude::testing::HELL_API_TEST;

    #[tokio::test]
    async fn planets_endpoint() {
        let result = HELL_API_TEST.planets().await;
        let inner = result.unwrap();
        let first = inner.first().unwrap();

        let result = HELL_API_TEST.planet(first.id).await;
        result.unwrap();
    }

    #[tokio::test]
    async fn planet_events_endpoint() {
        let result = HELL_API_TEST.planet_events().await;
        result.unwrap();
    }
}
