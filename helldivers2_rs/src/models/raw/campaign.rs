use helldivers2_rs_derive::parse_test;
use serde::Deserialize;

/// Represents information about an ongoing campaign.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
#[parse_test(make_parseable)]
pub struct Campaign {
    /// The identifier of this campaign.
    pub id: i32,
    /// The index of the referred planet.
    #[serde(rename = "planetIndex")]
    pub planet_index: i32,
    /// The indicator for the type of campaign (see helldivers-2/json).
    #[serde(rename = "type")]
    pub campaign_type: i32, // TODO enum type CampaignType
    /// The amount of campaigns the planet has seen.
    pub count: i32,
    // TODO race field
}

#[cfg(test)]
mod tests {
    use crate::{models::raw::campaign::Campaign, prelude::TestValue};

    impl TestValue for Campaign {
        const TEST_JSON: &'static str = r#"
            {
              "id": 50790,
              "planetIndex": 175,
              "type": 0,
              "count": 3,
              "race": 1
            }
        "#;

        fn test_expected() -> Self {
            Campaign {
                id: 50790,
                planet_index: 175,
                campaign_type: 0,
                count: 3,
            }
        }
    }
}
