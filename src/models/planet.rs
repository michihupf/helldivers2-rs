use chrono::{DateTime, Utc};

use super::{
    assignment::JointOperationId, campaign::CampaignId, dispatch::Message, stats::Statistics,
};

/// Represents the current status of a planet in the war.
pub struct PlanetStatus {
    /// The identifier of the corresponding PlanetInfo.
    index: i32,
    /// The faction currently owning the planet.
    owner: i32,
    /// The current liberation of a planet.
    health: i64,
    /// The health regeneration if the planet was left alone.
    regen_per_sec: f64,
    /// The amount of players active on this planet.
    players: u64,
}

/// Represents an attack on a planet.
pub struct PlanetAttack {
    /// The identifier of where the attack originates from.
    source: i32,
    /// The planet that is targeted by the attack.
    target: i32,
}

/// Represents an ongoing event on a planet.
pub struct PlanetEvent {
    /// The unique identifier of the event.
    id: i32,
    /// The index of the affected planet.
    planet_index: i32,
    /// The identifier indicating the type of the event.
    event_type: i32, // TODO enum type PlanetEventType
    /// The faction identifier that owns the planet.
    race: i32, // TODO enum type Race
    /// The current health of the event.
    health: i64,
    /// The maximum health of the event.
    max_health: i64,
    /// The time at which this event starts.
    start: DateTime<Utc>,
    /// The time at which this event ends.
    expire: DateTime<Utc>,
    /// The identifier of a related Campagin.
    campaign_id: CampaignId,
    /// A list of identifiers for related joint operations.
    joint_operations: Vec<JointOperationId>,
}

/// Represents an ongoing event on a planet
pub type Event = PlanetEvent;

/// Represents information of a planet.
pub struct PlanetInfo {
    /// The identifier for the planet.
    index: i32,
    /// Purpose is unknown at this point.
    settings_hash: i64,
    /// A set of X/Y coordinates specifying the position of the planet.
    position: PlanetCoordinates,
    /// A list of links to other planets (supply lines).
    waypoints: Vec<i32>,
    /// The identifier of the sector the planet is located in.
    sector: i32,
    /// The health of the planet.
    max_health: i64,
    /// Whether this planet is currently active in the galactic war.
    disabled: bool,
    /// The identifier of the faction that initially owned the planet.
    initial_owner: i32,
}

/// Represents the coordinates returned by the ArrowHead API.
pub struct PlanetCoordinates {
    x: f32,
    y: f32,
}

// Represents a position on the galatic war map.
pub type Position = PlanetCoordinates;

/// Represents information about the homeworld(s) of a given race (faction).
pub struct HomeWorld {
    race: i32,
    planet_indices: Vec<i32>,
}

/// Contains all aggregated information ArrowHead has about a planet.
pub struct Planet {
    /// The unique identifier ArrowHead assigned to this planet.
    index: i32,
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
    max_health: i64,
    /// The current health of the planet.
    health: i64,
    /// Whether the planet is disabled.
    disabled: bool,
    /// The faction that originally owned the planet.
    initial_owner: String,
    /// The faction that currently owns the planet.
    current_owner: String,
    /// How much the planet regenerates health when left alone.
    regen_per_second: f32,
    /// Information on the active event on this planet.
    event: Event,
    /// A set of statistics for this planet.
    statistics: Statistics,
    /// A list of planets currently attacked by this planet.
    attacking: Vec<i32>,
}

/// Represents information about a biome of a Planet.
pub struct Biome {
    // The name of the biome.
    name: String,
    /// A short human-readable description of the biome.
    description: String,
}

/// Represents an environmental hazard that can be present on a Planet.
pub struct Hazard {
    /// The name of the environmental hazard.
    name: String,
    /// The description of the environmental hazard.
    description: String,
}
