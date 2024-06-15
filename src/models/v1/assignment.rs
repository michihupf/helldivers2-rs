use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{
    middleware,
    models::common,
    prelude::{Parseable, Result},
    HellApi,
};

use super::dispatch::Message;

/// Represents an assignment ("Major Order") given by Super Earth to the community.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Assignment {
    /// The unique identifier of the assignment.
    pub id: i64,
    /// A list of numbers. How they represent progress is currently unknown.
    pub progress: Vec<i32>,
    /// The title of the assignment.
    pub title: Message,
    /// The briefing (long description) of the assignment. Usually contains
    /// context.
    pub briefing: Message,
    /// A short form description of the assignment.
    pub description: Message,
    /// A list of tasks that need to be completed for this major order.
    pub tasks: Vec<Task>,
    /// The reward for completing the assignment.
    pub reward: Reward,
    /// The date when the assignment will expire.
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

#[cfg(test)]
mod tests {
    use crate::{models::common::assignment::TaskType, prelude::Parseable};

    use super::Assignment;

    #[test]
    fn parse_assignment() {
        let json = r#"
              {
                "id": 3690749963,
                "progress": [
                  0,
                  0,
                  0,
                  0,
                  0
                ],
                "title": "MAJOR ORDER",
                "briefing": "Reduce the Terminid population and clear planets for citizen settlement, utilizing the new Hive Breaker Drill to cleanse their nurseries.",
                "description": "Liberate the designated planets.",
                "tasks": [
                  {
                    "type": 11,
                    "values": [
                          1,
                          1,
                          34
                        ],
                        "valueTypes": [
                          3,
                          11,
                          12
                        ]
                      },
                      {
                        "type": 11,
                        "values": [
                          1,
                          1,
                          211
                        ],
                        "valueTypes": [
                          3,
                          11,
                          12
                        ]
                      },
                      {
                        "type": 11,
                        "values": [
                          1,
                          1,
                          169
                        ],
                        "valueTypes": [
                          3,
                          11,
                          12
                        ]
                      },
                      {
                        "type": 11,
                        "values": [
                          1,
                          1,
                          78
                        ],
                        "valueTypes": [
                          3,
                          11,
                          12
                        ]
                      },
                      {
                        "type": 11,
                        "values": [
                          1,
                          1,
                          170
                        ],
                        "valueTypes": [
                          3,
                          11,
                          12
                        ]
                      }
                    ],
                    "reward": {
                      "type": 1,
                      "amount": 55
                    },
                    "expiration": "2024-06-22T15:54:52.2224108Z"
                  }"#;
        let json = serde_json::from_str(json).unwrap();

        let assignment = Assignment::parse(json).unwrap();
        for task in assignment.tasks {
            assert_eq!(task.task_type, TaskType::Liberation);
        }
    }
}
