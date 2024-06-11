use serde::Deserialize;

/// Represents information about an ongoing campaign.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
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
}
