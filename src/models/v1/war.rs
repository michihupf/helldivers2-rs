use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;

use crate::{
    middleware,
    prelude::{Parseable, Result},
    HellApi,
};

use super::stats::Statistics;

/// Global information about the ongoing war.
#[non_exhaustive]
#[serde_with::serde_as]
#[derive(Debug, Deserialize, PartialEq)]
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

impl Parseable for War {}

impl HellApi {
    /// Requests the the current war.
    ///
    /// Endpoint: `/api/v1/war`.
    pub async fn war() -> Result<War> {
        middleware::request_blocking("/api/v1/war").await
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;
    use const_format::formatcp;

    use crate::{
        models::v1::stats::Statistics,
        prelude::{Parseable, TestValue},
    };

    use super::War;

    impl TestValue for War {
        const TEST_JSON: &'static str = formatcp!(
            r#"{{
                "started": "2024-07-07T13:34:01.786Z",
                "ended": "2024-07-07T13:34:01.786Z",
                "now": "2024-07-07T13:34:01.786Z",
                "clientVersion": "0.1.0",
                "factions": [],
                "impactMultiplier": 0,  
                "statistics": {}
            }}"#,
            Statistics::TEST_JSON
        );

        fn test_expected() -> Self {
            War {
                started: NaiveDateTime::parse_from_str(
                    "2024-07-07T13:34:01.786Z",
                    "%Y-%m-%dT%H:%M:%S%.fZ",
                )
                .unwrap(),
                ended: NaiveDateTime::parse_from_str(
                    "2024-07-07T13:34:01.786Z",
                    "%Y-%m-%dT%H:%M:%S%.fZ",
                )
                .unwrap(),
                now: NaiveDateTime::parse_from_str(
                    "2024-07-07T13:34:01.786Z",
                    "%Y-%m-%dT%H:%M:%S%.fZ",
                )
                .unwrap(),
                client_version: String::from("0.1.0"),
                factions: vec![],
                impact_multiplier: 0f32,
                statistics: Statistics::test_expected(),
            }
        }
    }

    #[test]
    fn parse_war() {
        let json = serde_json::from_str(War::TEST_JSON).unwrap();
        let war = War::parse(json).unwrap();

        assert_eq!(war, War::test_expected());
    }
}
