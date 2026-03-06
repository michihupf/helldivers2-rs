use chrono::{DateTime, NaiveDateTime, Utc};
use proc::{parse_test, Parseable};
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
#[serde_with::serde_as]
#[derive(Debug, Deserialize, Parseable)]
#[parse_test]
pub struct MajorOrder {
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
    pub description: Option<Message>,
    /// A list of tasks that need to be completed for this major order.
    pub tasks: Vec<Task>,
    /// The reward for completing the assignment.
    pub reward: MajorOrderReward,
    /// The date when the assignment will expire.
    #[serde_as(as = "DateTime<Utc>")]
    pub expiration: NaiveDateTime,
}

impl Parseable for Vec<MajorOrder> {}

/// Represents a task of a `MajorOrder`.
pub type Task = common::assignment::Task;

/// Represents the reward of a `MajorOrder`.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
#[parse_test(make_parseable)]
pub struct MajorOrderReward {
    /// The type of reward (medals, super credits, etc.).
    pub r#type: i32,
    /// The amount the player will receive upon completion.
    pub amount: i32,
}

impl HellApi {
    impl_route!(
        "/api/v1/assignments",
        major_orders,
        Vec<MajorOrder>,
        "Requests current major orders."
    );

    /// Requests a specific major order.
    ///
    /// Endpoint: `/api/v1/assignments/{index}`
    pub async fn major_order(index: i64) -> Result<MajorOrder> {
        let endpoint = format!("/api/v1/assignments/{index}");
        middleware::request_blocking(endpoint.as_str()).await
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;
    use const_format::formatcp;

    use crate::{
        models::{
            common::assignment::TaskType,
            v1::{
                assignment::{MajorOrderReward, Task},
                dispatch::Message,
            },
        },
        prelude::TestValue,
    };

    use super::MajorOrder;

    impl TestValue for MajorOrderReward {
        const TEST_JSON: &'static str = r#"{
            "type": 1,
            "amount": 55
        }"#;

        fn test_expected() -> Self {
            MajorOrderReward {
                r#type: 1,
                amount: 55,
            }
        }
    }

    impl TestValue for MajorOrder {
        const TEST_JSON: &'static str = formatcp!(
            r#"{{
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
            "description": null,
            "tasks": [
              {{
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
              }}
            ],
            "reward": {},
            "expiration": "2024-06-22T15:54:52.2224108Z",
            "flags": 0
            }}"#,
            MajorOrderReward::TEST_JSON
        );

        fn test_expected() -> Self {
            MajorOrder {
                id: 3690749963,
                progress: vec![0, 0, 0, 0, 0],
                title: Message::Simple(String::from("MAJOR ORDER")),
                briefing: Message::Simple(String::from("Reduce the Terminid population and clear planets for citizen settlement, utilizing the new Hive Breaker Drill to cleanse their nurseries.")),
                description: None,
                tasks: vec!{
                  Task {task_type: TaskType::Liberation, values: vec!{1,1,34}, value_types: vec!{3,11,12}}
                },
                reward: MajorOrderReward::test_expected(),
                expiration: NaiveDateTime::parse_from_str("2024-06-22T15:54:52.2224108Z",
                "%Y-%m-%dT%H:%M:%S%.fZ").unwrap(),
            }
        }
    }
}
