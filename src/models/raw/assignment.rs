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
#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
pub struct Assignment {
    /// An internal identifier for this assignment.
    id32: i64,
    /// A list of numbers. How they represent progress is currently unknown.
    progress: Vec<i32>,
    /// The amount of seconds until this assignment expires.
    #[serde(rename = "expiresIn")]
    #[serde_as(as = "DurationSeconds")]
    expires_in: Duration,
    /// Contains detailed information on this assignment like briefing,
    /// rewards, etc.
    setting: Setting,
}

impl Parseable<Vec<Assignment>> for Vec<Assignment> {}

/// Represents the details of an Assignment like rewards and requirements.
#[derive(Debug, Deserialize)]
pub struct Setting {
    /// The type of the assignment. Values are unknown at this point.
    #[serde(rename = "type")]
    _type: i32,
    /// The title of the assignment.
    #[serde(rename = "overrideTitle")]
    override_title: String,
    /// The briefing (description) of this assignment.
    #[serde(rename = "overrideBrief")]
    override_brief: String,
    /// A description of what is expected of Helldivers to complet the assignment.
    #[serde(rename = "taskDescription")]
    task_descriptions: String,
    /// A list of Tasks that describe the assignment requirements.
    tasks: Vec<Task>,
    /// Information about the reward that players will receive upon completion.
    reward: Reward,
    /// Flags suspected to be a binary OR'd value. The exact purpose is unknown as of now.
    flags: i32,
}

pub type Task = common::assignment::Task;

/// Represents the reward of an Assignment.
#[derive(Debug, Deserialize)]
pub struct Reward {
    /// The type of reward.
    #[serde(rename = "type")]
    _type: RewardType,
    /// An internal identifier of this Reward.
    id32: i32,
    /// The amount the player will receive upon completion.
    amount: i32,
}

/// The type of a Reward. Currently only one value is known.
#[repr(i32)]
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum RewardType {
    Medals = 1,
    Unknown(i32),
}

impl HellApi {
    /// Retrieves a list of currently active assignments (like Major Orders).
    ///
    /// Endpoint: `/raw/api/v2/Assignment/War/{war_id}`.
    pub async fn assignments_raw(&self, war_id: WarId) -> Result<Vec<Assignment>> {
        let endpoint = format!("/raw/api/v2/Assignment/War/{}", war_id.id);
        self.request_blocking(endpoint.as_str()).await
    }
}

#[cfg(test)]
mod testing {
    use crate::{models::raw::war::WarId, prelude::testing::HELL_API_TEST};

    #[tokio::test]
    async fn assignments_raw_endpoint() {
        let result = HELL_API_TEST.assignments_raw(WarId::from(801)).await;
        result.unwrap();
    }
}
