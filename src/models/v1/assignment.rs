use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{
    middleware,
    models::common,
    prelude::{Parseable, Result},
    HellApi,
};

use super::dispatch::Message;

/// Represents a "Major Order" given by Super Earth to the community.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Assignment {
    /// The unique identifier of the major order.
    pub id: i64,
    /// A list of numbers. How they represent progress is currently unknown.
    pub progress: Vec<i32>,
    /// The title of the major order.
    pub title: Message,
    /// The briefing (long description) of the major order. Usually contains
    /// context.
    pub briefing: Message,
    /// A short form description of the major order.
    pub description: Message,
    /// A list of tasks that need to be completed for this major order.
    pub tasks: Vec<Task>,
    /// The reward for completing the order.
    pub reward: Reward,
    /// The date when the major order will expire.
    pub expiration: DateTime<Utc>,
}

impl Parseable for Assignment {}
impl Parseable for Vec<Assignment> {}

/// Represents a task of an `Assignment`.
pub type Task = common::assignment::Task;

/// Represents the reward of an `Assignment`.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Reward {
    /// The type of reward (medals, super credits, etc.).
    #[serde(rename = "type")]
    pub reward_type: i32,
    /// The amount the player will receive upon completion.
    pub amount: i32,
}

impl HellApi {
    /// Requests current major orders.
    ///
    /// Endpoint: `/api/v1/assignments`.
    pub async fn assignments() -> Result<Vec<Assignment>> {
        middleware::request_blocking::<Vec<Assignment>>("/api/v1/assignments").await
    }

    /// Requests a specific major order.
    ///
    /// Endpoint: `/api/v1/assignments/{index}`
    pub async fn assignment(index: i64) -> Result<Assignment> {
        let endpoint = format!("/api/v1/assignments/{index}");
        middleware::request_blocking(endpoint.as_str()).await
    }
}
