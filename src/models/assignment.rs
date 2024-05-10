use std::time::Duration;

use chrono::{DateTime, Utc};

use super::dispatch::Message;

/// Represents an assignment given from Super Earth to the Helldivers.
pub struct Assignment {
    /// An internal identifier for this assignment.
    id32: i64,
    /// A list of numbers. How they represent progress is currently unknown.
    progress: Vec<i32>,
    /// The amount of seconds until this assignment expires.
    expires_in: Duration,
    /// Contains detailed information on this assignment like briefing,
    /// rewards, etc.
    setting: Setting,
}

/// Represents the details of an Assignment like rewards and requirements.
pub struct Setting {
    /// The type of the assignment. Values are unknown at this point.
    _type: i32,
    /// The title of the assignment.
    override_title: String,
    /// The briefing (description) of this assignment.
    override_brief: String,
    /// A description of what is expected of Helldivers to complet the assignment.
    task_descriptions: String,
    /// A list of Tasks that describe the assignment requirements.
    tasks: Vec<Task>,
    /// Information about the reward that players will receive upon completion.
    reward: Reward,
    /// Flags suspected to be a binary OR'd value. The exact purpose is unknown as of now.
    flags: i32,
}

/// Represents a task in an Assignment. Its exact values are not known and
/// little of its purpose is clear.
pub struct Task {
    /// Numerical value. Purpose unknown.
    _type: i32,
    /// A list of numerical values. Purpose unknown.
    values: Vec<i32>,
    /// A list of numerical values. Purpose unknown.
    value_types: Vec<i32>,
}

/// Represents the reward of an Assignment.
pub struct Reward {
    /// The type of reward.
    _type: RewardType,
    /// An internal identifier of this Reward.
    id32: i32,
    /// The amount the player will receive upon completion.
    amount: i32,
}

/// The type of a Reward. Currently only one value is known.
#[repr(i32)]
enum RewardType {
    Medals = 1,
    Unknown(i32),
}

/// Represents a "Major Order" given by Super Earth to the community.
pub struct MajorOrder {
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
    tasks: Vec<MajorOrderTask>,
    /// The reward for completing the order.
    reward: MajorOrderReward,
    /// The date when the major order will expire.
    expiration: DateTime<Utc>,
}

/// Represents a task of a MajorOrder.
pub type MajorOrderTask = Task;

/// Represents the reward of a MajorOrder.
pub struct MajorOrderReward {
    /// The type of reward (medals, super credits, etc.).
    _type: i32,
    /// The amount the player will receive upon completion.
    amount: i32,
}

pub type JointOperationId = i32;
