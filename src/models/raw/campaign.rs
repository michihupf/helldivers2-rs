use serde::Deserialize;

/// Represents information about an ongoing campaign.
#[derive(Debug, Deserialize)]
pub struct Campaign {
    /// The identifier of this campaign.
    id: i32,
    /// The index of the referred planet.
    #[serde(rename = "planetIndex")]
    planet_index: i32,
    /// The indicator for the type of campaign (see helldivers-2/json).
    #[serde(rename = "type")]
    campaign_type: i32, // TODO enum type CampaignType
    /// The amount of campaigns the planet has seen.
    count: i32,
}
