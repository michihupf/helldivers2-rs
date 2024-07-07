use serde::Deserialize;

use crate::{
    middleware,
    prelude::{Parseable, Result},
    HellApi,
};

use super::planet::Planet;

/// Represents an ongoing campaign on a planet.
#[non_exhaustive]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Campaign {
    /// The unique identifier of this campaign.
    pub id: i32,
    /// The planet on which the campaign takes place.
    pub planet: Planet,
    /// The type of campaign. TODO This should be mapped onto an enum.
    #[serde(rename = "type")]
    pub _type: i32,
    /// Indicates how many campaigns have already been fought on this planet.
    pub count: u64,
}

impl Parseable for Campaign {}
impl Parseable for Vec<Campaign> {}

impl HellApi {
    /// Retrieves a list of all available campaign information.
    ///
    /// Endpoint: `/api/v1/campaigns`.
    pub async fn campaigns() -> Result<Vec<Campaign>> {
        middleware::request_blocking("/api/v1/campaigns").await
    }

    /// Retrieves a specific campaign with identifier `id`.
    ///
    /// Endpoint: `/api/v1/campaigns/{id}`.
    pub async fn campaign(id: i32) -> Result<Campaign> {
        let endpoint = format!("/api/v1/campaigns/{id}");
        middleware::request_blocking(endpoint.as_str()).await
    }
}

#[cfg(test)]
mod tests {
    use const_format::formatcp;

    use crate::{
        models::v1::planet::Planet,
        prelude::{Parseable, TestValue},
    };

    use super::Campaign;

    impl TestValue for Campaign {
        fn test_expected() -> Self {
            Campaign {
                id: 0,
                planet: Planet::test_expected(),
                _type: 1,
                count: 2,
            }
        }

        const TEST_JSON: &'static str = formatcp!(
            r#"{{
                "id": 0,
                "planet": {},
                "type": 1,
                "count": 2
            }}"#,
            Planet::TEST_JSON
        );
    }

    #[test]
    fn parse_campaign() {
        let json = serde_json::from_str(Campaign::TEST_JSON).unwrap();
        let campaign = Campaign::parse(json).unwrap();

        assert_eq!(campaign, Campaign::test_expected());
    }
}
