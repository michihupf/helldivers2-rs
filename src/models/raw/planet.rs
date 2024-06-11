use serde::Deserialize;

use crate::models::common;

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

/// Represents an ongoing even on a planet.
pub type PlanetEvent = common::planet::Event;

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
