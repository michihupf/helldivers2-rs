use std::time::Duration;

use serde::Deserialize;
use serde_with::DurationSeconds;

use crate::{
    models::common,
    prelude::{Parseable, Result},
    HellApi,
};

use super::war::WarId;

/// Represents an assignment given from Super Earth to the Helldivers.
#[non_exhaustive]
#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
pub struct Assignment {
    /// An internal identifier for this assignment.
    pub id32: i64,
    /// A list of numbers. How they represent progress is currently unknown.
    pub progress: Vec<i32>,
    /// The amount of seconds until this assignment expires.
    #[serde(rename = "expiresIn")]
    #[serde_as(as = "DurationSeconds")]
    pub expires_in: Duration,
    /// Contains detailed information on this assignment like briefing,
    /// rewards, etc.
    pub setting: Setting,
}

impl Parseable<Vec<Assignment>> for Vec<Assignment> {}

/// Represents the details of an Assignment like rewards and requirements.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Setting {
    /// The type of the assignment. Values are unknown at this point.
    #[serde(rename = "type")]
    pub _type: i32,
    /// The title of the assignment.
    #[serde(rename = "overrideTitle")]
    pub override_title: String,
    /// The briefing (description) of this assignment.
    #[serde(rename = "overrideBrief")]
    pub override_brief: String,
    /// A description of what is expected of Helldivers to complet the assignment.
    #[serde(rename = "taskDescription")]
    pub task_descriptions: String,
    /// A list of Tasks that describe the assignment requirements.
    pub tasks: Vec<Task>,
    /// Information about the reward that players will receive upon completion.
    pub reward: Reward,
    /// Flags suspected to be a binary OR'd value. The exact purpose is unknown as of now.
    pub flags: i32,
}

pub type Task = common::assignment::Task;

/// Represents the reward of an Assignment.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Reward {
    /// The type of reward.
    #[serde(rename = "type")]
    pub _type: RewardType,
    /// An internal identifier of this Reward.
    pub id32: i32,
    /// The amount the player will receive upon completion.
    pub amount: i32,
}

/// The type of a Reward. Currently only one value is known.
#[non_exhaustive]
#[repr(i32)]
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RewardType {
    Medals = 1,
    Unknown(i32),
}

impl HellApi {
    /// Retrieves a list of currently active assignments (like Major Orders).
    ///
    /// Endpoint: `/raw/api/v2/Assignment/War/{war_id}`.
    pub async fn assignments_raw(war_id: WarId) -> Result<Vec<Assignment>> {
        let endpoint = format!("/raw/api/v2/Assignment/War/{}", war_id.id);
        Self::request_blocking(endpoint.as_str()).await
    }
}
