use std::time::Duration;

use serde::Deserialize;
use serde_with::DurationSeconds;

/// Represents base statistics.
#[non_exhaustive]
#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
pub struct Statistics {
    /// The amount of missions won.
    #[serde(rename = "missionsWon")]
    pub missions_won: u64,
    /// The amount of missiosn lost.
    #[serde(rename = "missionsLost")]
    pub missions_lost: u64,
    /// The amount of time spent in missions.
    #[serde(rename = "missionTime")]
    #[serde_as(as = "DurationSeconds")]
    pub mission_time: Duration,
    /// The amount of bugs killed since the start of the season.
    #[serde(rename = "terminidKills")]
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
    #[serde_as(as = "DurationSeconds")]
    pub time_played: Duration,
    /// The mount of casualties on the side of humanity.
    pub deaths: u64,
    /// The amount of revives.
    pub revives: u64,
    /// The amount of friendly fire casualties.
    pub friendlies: u64,
    /// A percentage 0-100 indicating how many started missions ended in success.
    #[serde(rename = "missionSuccessRate")]
    pub mission_success_rate: u8,
    /// A percentage 0-100 indicating the average accuracy of Helldivers.
    pub accuracy: u8,
    /// The player count at the time of the snapshot.
    #[serde(rename = "playerCount")]
    pub player_count: u64,
}
