use std::time::Duration;

use helldivers2_rs_derive::parse_test;
use serde::Deserialize;
use serde_with::DurationSeconds;

/// Galaxy wide statistics aggregated from all planets.
#[non_exhaustive]
#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
#[parse_test(make_parseable)]
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
#[parse_test(make_parseable)]
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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{
        models::raw::stats::{GalaxyStats, PlanetStats},
        prelude::TestValue,
    };

    impl TestValue for GalaxyStats {
        const TEST_JSON: &'static str = r#"{
            "missionsWon": 854923839,
            "missionsLost": 83770325,
            "missionTime": 2773446319105,
            "bugKills": 191083890470,
            "automatonKills": 103682285336,
            "illuminateKills": 58494902999,
            "bulletsFired": 1653595526857,
            "bulletsHit": 1792220423767,
            "timePlayed": 2773446319105,
            "deaths": 7547297114,
            "revives": 2,
            "friendlies": 863438872,
            "missionSuccessRate": 91,
            "accurracy": 100
        }"#;

        fn test_expected() -> Self {
            GalaxyStats {
                missions_won: 854923839,
                missions_lost: 83770325,
                mission_time: Duration::from_secs(2773446319105),
                bug_kills: 191083890470,
                automaton_kills: 103682285336,
                illumintate_kills: 58494902999,
                bullets_fired: 1653595526857,
                bullets_hit: 1792220423767,
                time_played: Duration::from_secs(2773446319105),
                deaths: 7547297114,
                revives: 2,
                friendlies: 863438872,
                mission_success_rate: 91,
                accuracy: 100,
            }
        }
    }

    impl TestValue for PlanetStats {
        const TEST_JSON: &'static str = r#"{
          "planetIndex": 1,
          "missionsWon": 92,
          "missionsLost": 31,
          "missionTime": 294781,
          "bugKills": 23841,
          "automatonKills": 4069,
          "illuminateKills": 21348513,
          "bulletsFired": 200184202,
          "bulletsHit": 217262,
          "timePlayed": 294781,
          "deaths": 918,
          "revives": 0,
          "friendlies": 89,
          "missionSuccessRate": 74,
          "accurracy": 0
        }"#;

        fn test_expected() -> Self {
            PlanetStats {
                planet_index: 1,
                missions_won: 92,
                missions_lost: 31,
                mission_time: Duration::from_secs(294781),
                bug_kills: 23841,
                automaton_kills: 4069,
                illumintate_kills: 21348513,
                bullets_fired: 200184202,
                bullets_hit: 217262,
                time_played: Duration::from_secs(294781),
                deaths: 918,
                revives: 0,
                friendlies: 89,
                mission_success_rate: 74,
                accuracy: 0,
            }
        }
    }
}
