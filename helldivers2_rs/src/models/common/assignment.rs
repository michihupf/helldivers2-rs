use serde::Deserialize;
use serde_repr::Deserialize_repr;

/// Represents a task type for an Assignment. Its exact values are not
/// known but some have been found.
#[non_exhaustive]
#[repr(i32)]
#[derive(Debug, Clone, PartialEq, Deserialize_repr)]
pub enum TaskType {
    /// Eradicate the opposing faction.
    Eradication = 3,
    /// Liberate the supplied planets.
    Liberation = 11,
    /// Defend against opposing forces.
    Defense,
    /// FIXME More information needed.
    Control,
    /// Unknown type. If you figure it out feel free to raise an issue.
    #[serde(other)]
    Unknown,
}

/// Represents a task in an Assignment. Its exact values are not known and
/// little of its purpose is clear.
#[non_exhaustive]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Task {
    /// Numerical value. Only some values are known
    #[serde(rename = "type")]
    pub task_type: TaskType,
    /// A list of numerical values. Purpose unknown. FIXME
    pub values: Vec<u64>,
    /// A list of numerical values. Purpose unknown. FIXME
    #[serde(rename = "valueTypes")]
    pub value_types: Vec<u64>,
}

pub type JointOperationId = i32;
pub type CampaignId = i32;
