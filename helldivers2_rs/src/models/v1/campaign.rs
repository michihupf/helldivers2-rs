use helldivers2_rs_derive::{Parseable, parse_test};
use serde::Deserialize;

use crate::{
    HellApi, middleware,
    prelude::{Parseable, Result},
};

use super::planet::Planet;

/// Represents an ongoing campaign on a planet.
#[non_exhaustive]
#[derive(Debug, Deserialize, Parseable)]
#[parse_test]
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
    /// The faction that is currently fighting this campaign.
    pub faction: String,
}

impl Parseable for Vec<Campaign> {}

impl HellApi {
    impl_route!(
        "/api/v1/campaigns",
        campaigns,
        Vec<Campaign>,
        "Fetches a list of all available campaigns."
    );

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

    use crate::{models::v1::planet::Planet, prelude::TestValue};

    use super::Campaign;

    impl TestValue for Campaign {
        fn test_expected() -> Self {
            Campaign {
                id: 0,
                planet: Planet::test_expected(),
                _type: 1,
                count: 2,
                faction: String::from("testing"),
            }
        }

        const TEST_JSON: &'static str = formatcp!(
            r#"{{
                "id": 0,
                "planet": {},
                "type": 1,
                "count": 2,
                "faction": "testing"
            }}"#,
            Planet::TEST_JSON
        );
    }
}
