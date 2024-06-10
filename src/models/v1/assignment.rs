use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{
    models::common::{self},
    prelude::{Parseable, Result},
    HellApi,
};

use super::dispatch::Message;

/// Represents a "Major Order" given by Super Earth to the community.
#[derive(Debug, Deserialize)]
pub struct Assignment {
    /// The unique identifier of the major order.
    id: i64,
    /// A list of numbers. How they represent progress is currently unknown.
    progress: Vec<i32>,
    /// The title of the major order.
    title: Message,
    /// The briefing (long description) of the major order. Usually contains
    /// context.
    briefing: Message,
    /// A short form description of the major order.
    description: Message,
    /// A list of tasks that need to be completed for this major order.
    tasks: Vec<Task>,
    /// The reward for completing the order.
    reward: Reward,
    /// The date when the major order will expire.
    expiration: DateTime<Utc>,
}

impl Parseable<Assignment> for Assignment {}
impl Parseable<Vec<Assignment>> for Vec<Assignment> {}

/// Represents a task of an `Assignment`.
pub type Task = common::assignment::Task;

/// Represents the reward of an `Assignment`.
#[derive(Debug, Deserialize)]
pub struct Reward {
    /// The type of reward (medals, super credits, etc.).
    #[serde(rename = "type")]
    reward_type: i32,
    /// The amount the player will receive upon completion.
    amount: i32,
}

impl HellApi {
    /// Requests current major orders.
    ///
    /// Endpoint: `/api/v1/assignments`.
    pub async fn assignments(&self) -> Result<Vec<Assignment>> {
        self.request_blocking::<Vec<Assignment>>("/api/v1/assignments")
            .await
    }

    /// Requests a specific major order.
    ///
    /// Endpoint: `/api/v1/assignments/{index}`
    pub async fn assignment(&self, index: i64) -> Result<Assignment> {
        let endpoint = format!("/api/v1/assignments/{index}");
        self.request_blocking(endpoint.as_str()).await
    }
}

#[cfg(test)]
mod testing {
    use crate::prelude::testing::HELL_API_TEST;

    #[tokio::test]
    async fn major_orders_endpoint() {
        let result = HELL_API_TEST.assignments().await;
        let inner = result.unwrap();
        let first = inner.first().unwrap();

        let result = HELL_API_TEST.assignment(first.id).await;
        result.unwrap();
    }
}
