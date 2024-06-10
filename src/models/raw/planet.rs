use serde::Deserialize;

use crate::models::common;

/// Represents information about the homeworld(s) of a given race (faction).
#[derive(Debug, Deserialize)]
pub struct HomeWorld {
    race: i32,
    /// A list of planet index identifiers.
    #[serde(rename = "planetIndices")]
    planet_indices: Vec<i32>,
}

/// Represents the current status of a planet in the war.
#[derive(Debug, Deserialize)]
pub struct PlanetStatus {
    /// The identifier of the corresponding PlanetInfo.
    index: i32,
    /// The faction currently owning the planet.
    owner: i32,
    /// The current liberation of a planet.
    health: i64,
    /// The health regeneration if the planet was left alone.
    #[serde(rename = "regenPerSecond")]
    regen_per_sec: f64,
    /// The amount of players active on this planet.
    players: u64,
}

/// Represents an attack on a planet.
#[derive(Debug, Deserialize)]
pub struct PlanetAttack {
    /// The identifier of where the attack originates from.
    source: i32,
    /// The planet that is targeted by the attack.
    target: i32,
}

/// Represents an ongoing even on a planet.
pub type PlanetEvent = common::planet::Event;

/// Represents the coordinates returned by the ArrowHead API.
pub type PlanetCoordinates = common::planet::Position;

/// Represents information of a planet.
#[derive(Debug, Deserialize)]
pub struct PlanetInfo {
    /// The identifier for the planet.
    index: i32,
    /// Purpose is unknown at this point.
    #[serde(rename = "settingsHash")]
    settings_hash: i64,
    /// A set of X/Y coordinates specifying the position of the planet.
    position: PlanetCoordinates,
    /// A list of links to other planets (supply lines).
    waypoints: Vec<i32>,
    /// The identifier of the sector the planet is located in.
    sector: i32,
    /// The health of the planet.
    #[serde(rename = "maxHealth")]
    max_health: i64,
    /// Whether this planet is currently active in the galactic war.
    disabled: bool,
    /// The identifier of the faction that initially owned the planet.
    #[serde(rename = "initialOwner")]
    initial_owner: i32,
}
