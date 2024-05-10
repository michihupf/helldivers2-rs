use std::borrow::Borrow;

use chrono::{DateTime, Utc};

use super::{
    campaign::Campaign,
    planet::{HomeWorld, PlanetAttack, PlanetEvent, PlanetInfo, PlanetStatus},
    stats::{GalaxyStats, PlanetStats, Statistics},
};

/// Type of the ID returned from the WarID endpoint.
#[repr(transparent)]
pub struct WarId {
    id: i32,
}

impl WarId {
    pub async fn current() -> Result<i32, reqwest::Error> {
        let raw = reqwest::get("https://api.helldivers2.dev/raw/api/WarSeason/current/WarID")
            .await?
            .text()
            .await?;

        let json = json::parse(&raw).unwrap();

        todo!();
        Ok(1)
    }
}

/// Represents a snapshot of the current status of the
/// galactic war.
pub struct WarStatus {
    /// The war season this status refers to.
    war_id: WarId,
    /// The time the snapshot was taken.
    time: DateTime<Utc>,
    /// The factor by which influence at mission end is
    /// multiplied to calculate the mission impact on
    /// liberation.
    impact_multiplier: f32,
    /// Internal identifier. Purpose unknown.
    story_beat_id32: i64,
    /// A list of planet statuses.
    planet_status: Vec<PlanetStatus>,
    /// A list of current planet attacks.
    planet_attacks: Vec<PlanetAttack>,
    /// A list of ongoing campaigns in the war.
    campaigns: Vec<Campaign>,
    /// A list of current planet events.
    planet_events: Vec<PlanetEvent>,
}

/// Represents information about the current war.
pub struct WarInfo {
    /// The war season this WarInfo refers to.
    war_id: WarId,
    /// The start time of the season.
    start: DateTime<Utc>,
    /// The end time of the season.
    end: DateTime<Utc>,
    /// A version string that indicates the minimum game client version
    /// the API supports.
    minimum_client_version: String,
    /// A list of planets that are involved in the war.
    planet_infos: Vec<PlanetInfo>,
    /// A list of homeworlds for the races (factions) invld in the war.
    home_worlds: Vec<HomeWorld>,
}

/// Represents general statistics about the galaxy and specific planets.
pub struct WarSummary {
    /// Galaxy wide statistics aggregated from all planets.
    galaxy_stats: GalaxyStats,
    /// Statistics for specific planets.
    planet_stats: PlanetStats,
}

/// Global information about the ongoing war.
pub struct War {
    /// When this war was started as a datetime String
    started: String,
    /// When the war will end (or has ended) as a datetime String,
    ended: String,
    /// The time the snapshot of the war was taken.
    now: String,
    /// The minimum client version required to play in this war.
    client_version: String,
    /// A list of faction names involved in the war
    factions: Vec<String>,
    /// A fraction used to calculate the impact of a mission on the war effort.
    impact_multiplier: f32,
    /// The statistics available for the galaxy wide war.
    statistics: Statistics,
}
