use chrono::NaiveDateTime;
use serde::Deserialize;
use serde_with::TimestampSeconds;

use crate::{
    prelude::{Parseable, Result},
    HellApi,
};

use super::war::WarId;

/// Represents an item in the newsfeed of Super Earth.
#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
pub struct NewsFeedItem {
    /// The identifier of this newsfeed item.
    pub id: i32,
    /// The time when this item was published
    #[serde_as(as = "TimestampSeconds")]
    pub published: NaiveDateTime,
    /// A numerical type. Purpose is unknown.
    #[serde(rename = "type")]
    pub _type: i32,
    /// The message.
    pub message: String,
}

impl Parseable<Vec<NewsFeedItem>> for Vec<NewsFeedItem> {}

impl HellApi {
    /// Retrieves a list of news messages from Super Earth.
    ///
    /// Endpoint: `/raw/api/NewsFeed/{war_id}`.
    pub async fn news_feed(&self, war_id: WarId) -> Result<Vec<NewsFeedItem>> {
        let endpoint = format!("/raw/api/NewsFeed/{}", war_id.id);
        self.request_blocking(endpoint.as_str()).await
    }
}

#[cfg(test)]
mod testing {
    use crate::{models::raw::war::WarId, prelude::testing::HELL_API_TEST};

    #[tokio::test]
    async fn news_feed_endpoint() {
        let result = HELL_API_TEST.news_feed(WarId::from(801)).await;
        result.unwrap();
    }
}
