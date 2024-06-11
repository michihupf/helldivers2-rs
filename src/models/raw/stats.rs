use std::time::Duration;

use serde::Deserialize;
use serde_with::DurationSeconds;

/// Galaxy wide statistics aggregated from all planets.
#[non_exhaustive]
#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
pub struct GalaxyStats {
    /// The amount of missions won.
    #[serde(rename = "missionsWon")]
    pub missions_won: u64,
    /// The amount of missiosn lost.
    #[serde(rename = "missionsLost")]
    pub missions_lost: u64,
    /// The amount of time spent in missions.
    #[serde(rename = "missionTime")]
    #[serde_as(as = "DurationSeconds<u64>")]
    pub mission_time: Duration,
    /// The amount of bugs killed since the start of the season.
    #[serde(rename = "bugKills")]
    pub bug_kills: u64,
    /// The amount of automatons killed since the start of the season.
    #[serde(rename = "automatonKills")]
    pub automaton_kills: u64,
    /// The amount of illuminate killed since the start of the season.
    #[serde(rename = "illuminateKills")]
    pub illumintate_kills: u64,
    /// The amount of bullets fired.
    #[serde(rename = "bulletsFired")]
    pub bullets_fired: u64,
    /// The amount of bullets hit.
    #[serde(rename = "bulletsHit")]
    pub bullets_hit: u64,
    /// The amount of time spent playing (including off-planet).
    #[serde(rename = "timePlayed")]
    #[serde_as(as = "DurationSeconds<u64>")]
    pub time_played: Duration,
    /// The mount of casualties on the side of humanity.
    pub deaths: u64,
    /// The amount of revives.
    pub revives: u64,
    /// The amount of friendly fire casualties.
    pub friendlies: u64,
    /// A percentage 0-100 indicating how many started missions ended in success.
    #[serde(rename = "missionSuccessRate")]
    pub mission_success_rate: u64,
    /// A percentage 0-100 indicating the average accuracy of Helldivers.
    #[serde(rename = "accurracy")] // typo in the API
    pub accuracy: u64,
}

/// Represents planet specific statistics.
#[non_exhaustive]
#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
pub struct PlanetStats {
    /// The index for which planet these stats are.
    #[serde(rename = "planetIndex")]
    pub planet_index: i32,
    /// The amount of missions won.
    #[serde(rename = "missionsWon")]
    pub missions_won: u64,
    /// The amount of missiosn lost.
    #[serde(rename = "missionsLost")]
    pub missions_lost: u64,
    /// The amount of time spent in missions.
    #[serde(rename = "missionTime")]
    #[serde_as(as = "DurationSeconds<u64>")]
    pub mission_time: Duration,
    /// The amount of bugs killed since the start of the season.
    #[serde(rename = "bugKills")]
    pub bug_kills: u64,
    /// The amount of automatons killed since the start of the season.
    #[serde(rename = "automatonKills")]
    pub automaton_kills: u64,
    /// The amount of illuminate killed since the start of the season.
    #[serde(rename = "illuminateKills")]
    pub illumintate_kills: u64,
    /// The amount of bullets fired.
    #[serde(rename = "bulletsFired")]
    pub bullets_fired: u64,
    /// The amount of bullets hit.
    #[serde(rename = "bulletsHit")]
    pub bullets_hit: u64,
    /// The amount of time spent playing (including off-planet).
    #[serde(rename = "timePlayed")]
    #[serde_as(as = "DurationSeconds<u64>")]
    pub time_played: Duration,
    /// The mount of casualties on the side of humanity.
    pub deaths: u64,
    /// The amount of revives.
    pub revives: u64,
    /// The amount of friendly fire casualties.
    pub friendlies: u64,
    /// A percentage 0-100 indicating how many started missions ended in success.
    #[serde(rename = "missionSuccessRate")]
    pub mission_success_rate: u64,
    /// A percentage 0-100 indicating the average accuracy of Helldivers.
    #[serde(rename = "accurracy")] // typo in the API
    pub accuracy: u64,
}
