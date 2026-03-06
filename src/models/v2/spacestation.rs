use chrono::{DateTime, NaiveDateTime, Utc};
use proc::parse_test;
use serde::Deserialize;

use crate::{
    middleware,
    models::v1::planet::Planet,
    prelude::{Parseable, Result},
    HellApi,
};

/// Represents the "cost" of a `TacticalAction`.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
#[parse_test]
pub struct TacticalActionCost {
    pub id: String,
    #[serde(rename = "itemMixId")]
    pub item_mix_id: i64,
    #[serde(rename = "targetValue")]
    pub target_value: i64,
    #[serde(rename = "currentValue")]
    pub current_value: f64,
    #[serde(rename = "deltaPerSecond")]
    pub delta_per_second: f64,
    #[serde(rename = "maxDonationAmmount")]
    pub max_donation_amount: i64,
    #[serde(rename = "maxDonationPeriodSeconds")]
    pub max_donation_period_secs: i64,
}

/// Represents a tactical action a `SpaceStation` can take.
#[non_exhaustive]
#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
#[parse_test]
pub struct TacticalAction {
    #[serde(rename = "id32")]
    pub id: i64,
    #[serde(rename = "mediaId32")]
    pub media_id: i64,
    pub name: String,
    pub description: String,
    #[serde(rename = "strategicDescription")]
    pub strategic_description: String,
    pub status: i32,
    #[serde(rename = "statusExpire")]
    #[serde_as(as = "DateTime<Utc>")]
    pub status_expire: NaiveDateTime,
    pub costs: Vec<TacticalActionCost>,
    #[serde(rename = "effectIds")]
    pub effect_ids: Vec<i32>,
}

/// Represents a Super Earth Democracy Space Station.
#[non_exhaustive]
#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
#[parse_test]
pub struct SpaceStation {
    /// The unique identifier of the station.
    #[serde(rename = "id32")]
    pub id: i64,
    /// The planet it is currently orbiting.
    pub planet: Planet,
    /// When the election for the next planet will end.
    #[serde(rename = "electionEnd")]
    #[serde_as(as = "DateTime<Utc>")]
    pub election_end: NaiveDateTime,
    /// A set of flags, purpose unknown.
    pub flags: i32,
    /// A list of tactical actions the space station supports.
    #[serde(rename = "tacticalActions")]
    pub tactical_actions: Vec<TacticalAction>,
}

impl Parseable for SpaceStation {}
impl Parseable for Vec<SpaceStation> {}

impl HellApi {
    impl_route!(
        "/api/v2/space-stations",
        space_stations,
        Vec<SpaceStation>,
        "Fetches a list of all available `SpaceStation`s."
    );

    /// Fetches a specific space station by index.
    ///
    /// Endpoint: `/api/v2/space-stations/{index}`.
    pub async fn space_station(id: i64) -> Result<SpaceStation> {
        middleware::request_blocking(format!("/api/v2/space-stations/{id}").as_str()).await
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;
    use const_format::formatcp;

    use crate::{
        models::{
            v1::planet::Planet,
            v2::spacestation::{SpaceStation, TacticalAction, TacticalActionCost},
        },
        prelude::{Parseable, TestValue},
    };

    impl Parseable for TacticalActionCost {}
    impl TestValue for TacticalActionCost {
        const TEST_JSON: &'static str = r#"
            {
              "id": "ce60caf8-d89e-ef11-88d0-002248533197",
              "itemMixId": 3992382197,
              "targetValue": 86400,
              "currentValue": 31100,
              "deltaPerSecond": 1,
              "maxDonationAmmount": 0,
              "maxDonationPeriodSeconds": 86400
            }
        "#;

        fn test_expected() -> Self {
            TacticalActionCost {
                id: String::from("ce60caf8-d89e-ef11-88d0-002248533197"),
                item_mix_id: 3992382197,
                target_value: 86400,
                current_value: 31100f64,
                delta_per_second: 1f64,
                max_donation_amount: 0,
                max_donation_period_secs: 86400,
            }
        }
    }

    impl Parseable for TacticalAction {}
    impl TestValue for TacticalAction {
        const TEST_JSON: &'static str = r#"
        {
          "id32": 4091660627,
          "mediaId32": 4091660627,
          "name": "EAGLE STORM",
          "description": "DSS crews maintain deployment of a 24-hour rotating fleet of Eagle Fighters from the massive three-tiered Convocation Bay, supporting all Helldiver operations on the planet with close air support and halting enemy offensives for a short time.",
          "strategicDescription": "<span data-ah=\"1\">Eagle Airstrikes</span> during missions. Slows enemy progress in <span data-ah=\"1\">Defense Campaigns</span>.",
          "status": 1,
          "statusExpire": "2026-03-10T04:19:41Z",
          "costs": [],
          "effectIds": [
            1209,
            1212,
            1216
          ]
        }
      "#;

        fn test_expected() -> Self {
            TacticalAction {
                id: 4091660627,
                media_id: 4091660627,
                name: String::from("EAGLE STORM"),
                description: String::from("DSS crews maintain deployment of a 24-hour rotating fleet of Eagle Fighters from the massive three-tiered Convocation Bay, supporting all Helldiver operations on the planet with close air support and halting enemy offensives for a short time."),
                strategic_description: String::from("<span data-ah=\"1\">Eagle Airstrikes</span> during missions. Slows enemy progress in <span data-ah=\"1\">Defense Campaigns</span>."),
                status: 1,
                status_expire: NaiveDateTime::parse_from_str("2026-03-10T04:19:41Z", "%Y-%m-%dT%H:%M:%SZ").unwrap(),
                costs: vec![],
                effect_ids: vec![1209, 1212, 1216],
            }
        }
    }

    impl TestValue for SpaceStation {
        const TEST_JSON: &'static str = formatcp!(
            r#"{{
              "id32": 749875195,
              "planet": {},
              "electionEnd": "2026-03-06T16:49:31Z",
              "flags": 1,
              "tacticalActions": []
            }}"#,
            Planet::TEST_JSON,
        );

        fn test_expected() -> Self {
            SpaceStation {
                id: 749875195,
                planet: Planet::test_expected(),
                election_end: NaiveDateTime::parse_from_str(
                    "2026-03-06T16:49:31Z",
                    "%Y-%m-%dT%H:%M:%SZ",
                )
                .unwrap(),
                flags: 1,
                tactical_actions: vec![],
            }
        }
    }
}
