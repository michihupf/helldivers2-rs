/// Represents information about an ongoing campaign.
pub struct Campaign {
    /// The identifier of this campaign.
    id: i32,
    /// The index of the referred planet.
    planet_index: i32,
    /// The indicator for the type of campaign (see helldivers-2/json).
    campaign_type: i32, // TODO enum type CampaignType
    /// The amount of campaigns the planet has seen.
    count: i32,
}

/// The type of a Campaign identifier.
pub type CampaignId = i32;
