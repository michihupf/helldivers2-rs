use std::{num::ParseIntError, str::FromStr};

use serde::Deserialize;
use serde_with::DisplayFromStr;

/// Represents a task type for an Assignment. Its exact values are not
/// known but some have been found.
#[non_exhaustive]
#[repr(i32)]
#[derive(Debug, Clone, Deserialize)]
pub enum TaskType {
    /// Eradicate the opposing faction.
    Eradication = 3,
    /// Liberate the supplied planets.
    Liberation = 11,
    /// Defend against opposing forces.
    Defense,
    /// FIXME More information needed.
    Control,
    /// Unknown type.
    Unknown(i32),
}

impl FromStr for TaskType {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaskType::*;

        let num = s.parse::<i32>()?;
        Ok(match num {
            3 => Eradication,
            11 => Liberation,
            12 => Defense,
            13 => Control,
            _ => Unknown(num),
        })
    }
}

/// Represents a task in an Assignment. Its exact values are not known and
/// little of its purpose is clear.
#[non_exhaustive]
#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
pub struct Task {
    /// Numerical value. Only some values are known
    #[serde(rename = "type")]
    #[serde_as(as = "DisplayFromStr")]
    task_type: TaskType,
    /// A list of numerical values. Purpose unknown. FIXME
    values: Vec<i32>,
    /// A list of numerical values. Purpose unknown. FIXME
    #[serde(rename = "valueTypes")]
    value_types: Vec<i32>,
}

pub type JointOperationId = i32;
pub type CampaignId = i32;
