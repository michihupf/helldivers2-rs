use serde::Deserialize;

/// Represents the coordinates returned by the ArrowHead API.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
