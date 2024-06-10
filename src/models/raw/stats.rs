use std::time::Duration;

use serde::Deserialize;
use serde_with::DurationSeconds;

/// Galaxy wide statistics aggregated from all planets.
#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
pub struct GalaxyStats {
    /// The amount of missions won.
    #[serde(rename = "missionsWon")]
    missions_won: u64,
    /// The amount of missiosn lost.
    #[serde(rename = "missionsLost")]
    missions_lost: u64,
    /// The amount of time spent in missions.
    #[serde(rename = "missionTime")]
    #[serde_as(as = "DurationSeconds<u64>")]
    mission_time: Duration,
    /// The amount of bugs killed since the start of the season.
    #[serde(rename = "bugKills")]
    bug_kills: u64,
    /// The amount of automatons killed since the start of the season.
    #[serde(rename = "automatonKills")]
    automaton_kills: u64,
    /// The amount of illuminate killed since the start of the season.
    #[serde(rename = "illuminateKills")]
    illumintate_kills: u64,
    /// The amount of bullets fired.
    #[serde(rename = "bulletsFired")]
    bullets_fired: u64,
    /// The amount of bullets hit.
    #[serde(rename = "bulletsHit")]
    bullets_hit: u64,
    /// The amount of time spent playing (including off-planet).
    #[serde(rename = "timePlayed")]
    #[serde_as(as = "DurationSeconds<u64>")]
    time_played: Duration,
    /// The mount of casualties on the side of humanity.
    deaths: u64,
    /// The amount of revives.
    revives: u64,
    /// The amount of friendly fire casualties.
    friendlies: u64,
    /// A percentage 0-100 indicating how many started missions ended in success.
    #[serde(rename = "missionSuccessRate")]
    mission_success_rate: u64,
    /// A percentage 0-100 indicating the average accuracy of Helldivers.
    #[serde(rename = "accurracy")] // typo in the API
    accuracy: u64,
}

/// Represents planet specific statistics.
#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
pub struct PlanetStats {
    /// The index for which planet these stats are.
    #[serde(rename = "planetIndex")]
    planet_index: i32,
    /// The amount of missions won.
    #[serde(rename = "missionsWon")]
    missions_won: u64,
    /// The amount of missiosn lost.
    #[serde(rename = "missionsLost")]
    missions_lost: u64,
    /// The amount of time spent in missions.
    #[serde(rename = "missionTime")]
    #[serde_as(as = "DurationSeconds<u64>")]
    mission_time: Duration,
    /// The amount of bugs killed since the start of the season.
    #[serde(rename = "bugKills")]
    bug_kills: u64,
    /// The amount of automatons killed since the start of the season.
    #[serde(rename = "automatonKills")]
    automaton_kills: u64,
    /// The amount of illuminate killed since the start of the season.
    #[serde(rename = "illuminateKills")]
    illumintate_kills: u64,
    /// The amount of bullets fired.
    #[serde(rename = "bulletsFired")]
    bullets_fired: u64,
    /// The amount of bullets hit.
    #[serde(rename = "bulletsHit")]
    bullets_hit: u64,
    /// The amount of time spent playing (including off-planet).
    #[serde(rename = "timePlayed")]
    #[serde_as(as = "DurationSeconds<u64>")]
    time_played: Duration,
    /// The mount of casualties on the side of humanity.
    deaths: u64,
    /// The amount of revives.
    revives: u64,
    /// The amount of friendly fire casualties.
    friendlies: u64,
    /// A percentage 0-100 indicating how many started missions ended in success.
    #[serde(rename = "missionSuccessRate")]
    mission_success_rate: u64,
    /// A percentage 0-100 indicating the average accuracy of Helldivers.
    #[serde(rename = "accurracy")] // typo in the API
    accuracy: u64,
}
