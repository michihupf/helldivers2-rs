use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;

use crate::{
    prelude::{Parseable, Result},
    HellApi,
};

use super::stats::Statistics;

/// Global information about the ongoing war.
#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
pub struct War {
    /// When this war was started as a datetime String
    #[serde_as(as = "DateTime<Utc>")]
    started: NaiveDateTime,
    /// When the war will end (or has ended) as a datetime String,
    #[serde_as(as = "DateTime<Utc>")]
    ended: NaiveDateTime,
    /// The time the snapshot of the war was taken.
    #[serde_as(as = "DateTime<Utc>")]
    now: NaiveDateTime,
    /// The minimum client version required to play in this war.
    #[serde(rename = "clientVersion")]
    client_version: String,
    /// A list of faction names involved in the war
    factions: Vec<String>,
    /// A fraction used to calculate the impact of a mission on the war effort.
    #[serde(rename = "impactMultiplier")]
    impact_multiplier: f32,
    /// The statistics available for the galaxy wide war.
    statistics: Statistics,
}

impl Parseable<War> for War {}

impl HellApi {
    /// Requests the the current war.
    ///
    /// Endpoint: `/api/v1/war`.
    pub async fn war(&self) -> Result<War> {
        self.request_blocking("/api/v1/war").await
    }
}

#[cfg(test)]
mod testing {
    use crate::prelude::testing::HELL_API_TEST;

    #[tokio::test]
    async fn war_endpoint() {
        let result = HELL_API_TEST.war().await;
        result.unwrap();
    }
}
