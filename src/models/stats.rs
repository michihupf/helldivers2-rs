use std::time::Duration;

/// Represents base statistics.
pub struct BaseStats {
    /// The amount of missions won.
    missions_won: u64,
    /// The amount of missiosn lost.
    missions_lost: u64,
    /// The amount of time spent in missions.
    mission_time: Duration,
    /// The amount of bugs killed since the start of the season.
    bug_kills: u64,
    /// The amount of automatons killed since the start of the season.
    automaton_kills: u64,
    /// The amount of illuminate killed since the start of the season.
    illumintate_kills: u64,
    /// The amount of bullets fired.
    bullets_fired: u64,
    /// The amount of bullets hit.
    bullets_hit: u64,
    /// The amount of time spent playing (including off-planet).
    time_played: Duration,
    /// The mount of casualties on the side of humanity.
    deaths: u64,
    /// The amount of revives.
    revives: u64,
    /// The amount of friendly fire casualties.
    friendlies: u64,
    /// A percentage (?) indicating how many started missions ended in success.
    mission_success_rate: u64,
    /// A percentage (?) indicating the average accuracy of Helldivers.
    accuracy: u64,
}

/// Represents galaxy wide statistics.
#[repr(transparent)]
pub struct GalaxyStats {
    /// The BaseStats for the galaxy.
    stats: BaseStats,
}

/// Represents planet specific statistics.
pub struct PlanetStats {
    /// The planet for which these stats are.
    planet_index: i32,
    /// The BaseStats for this planet.
    stats: BaseStats,
}

/// Represent a statistics snapshot.
pub struct Statistics {
    /// The player count at the time of the snapshot.
    player_count: u64,
    /// The BaseStats for this snapshot.
    stats: BaseStats,
}
