use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;

use crate::{
    prelude::{Parseable, Result},
    HellApi,
};

use super::stats::Statistics;

/// Global information about the ongoing war.
#[non_exhaustive]
#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
pub struct War {
    /// When this war was started as a datetime String
    #[serde_as(as = "DateTime<Utc>")]
    pub started: NaiveDateTime,
    /// When the war will end (or has ended) as a datetime String,
    #[serde_as(as = "DateTime<Utc>")]
    pub ended: NaiveDateTime,
    /// The time the snapshot of the war was taken.
    #[serde_as(as = "DateTime<Utc>")]
    pub now: NaiveDateTime,
    /// The minimum client version required to play in this war.
    #[serde(rename = "clientVersion")]
    pub client_version: String,
    /// A list of faction names involved in the war
    pub factions: Vec<String>,
    /// A fraction used to calculate the impact of a mission on the war effort.
    #[serde(rename = "impactMultiplier")]
    pub impact_multiplier: f32,
    /// The statistics available for the galaxy wide war.
    pub statistics: Statistics,
}

impl Parseable<War> for War {}

impl HellApi {
    /// Requests the the current war.
    ///
    /// Endpoint: `/api/v1/war`.
    pub async fn war() -> Result<War> {
        Self::request_blocking("/api/v1/war").await
    }
}
