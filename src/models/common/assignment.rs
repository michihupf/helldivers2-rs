use serde::Deserialize;

/// Represents a task in an Assignment. Its exact values are not known and
/// little of its purpose is clear.
#[derive(Debug, Deserialize)]
pub struct Task {
    /// Numerical value. Purpose unknown.
    #[serde(rename = "type")]
    task_type: i32,
    /// A list of numerical values. Purpose unknown.
    values: Vec<i32>,
    /// A list of numerical values. Purpose unknown.
    #[serde(rename = "valueTypes")]
    value_types: Vec<i32>,
}

pub type JointOperationId = i32;
pub type CampaignId = i32;
