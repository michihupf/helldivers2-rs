use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;

use crate::{
    prelude::{Parseable, Result},
    HellApi,
};

/// A message than can either be a simple String or a LocalizedMessage.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Message {
    Simple(String),
    Localized(LocalizedMessage),
}

/// A localized message for a specific language.
#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct LocalizedMessage {
    /// The message in en-US.
    #[serde(rename = "en-US")]
    en_US: Option<String>,
    /// The message in de-DE.
    #[serde(rename = "de-DE")]
    de_DE: Option<String>,
    /// The message in es-ES.
    #[serde(rename = "es-ES")]
    es_ES: Option<String>,
    /// The message in ru-RU.
    #[serde(rename = "ru-RU")]
    ru_RU: Option<String>,
    /// The message in fr-FR.
    #[serde(rename = "fr-FR")]
    fr_FR: Option<String>,
    /// The message in it-IT.
    #[serde(rename = "it-IT")]
    it_IT: Option<String>,
    /// The message in pl-PL.
    #[serde(rename = "pl-PL")]
    pl_PL: Option<String>,
    /// The message in zh-Hans.
    #[serde(rename = "zh-Hans")]
    zh_Hans: Option<String>,
    /// The message in zh-Hant.
    #[serde(rename = "zh-Hant")]
    zh_Hant: Option<String>,
}

/// Represents a message from high command to the players like updates on the
/// status of the war effort.
#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
pub struct Dispatch {
    /// The unique identifier of the dispatch.
    id: i32,
    /// The time when the dispatch was published.
    #[serde_as(as = "DateTime<Utc>")]
    published: NaiveDateTime,
    /// The type of dispatch. Purpose unknown as of now.
    #[serde(rename = "type")]
    _type: i32,
    /// The message that this dispatch represents.
    message: Message,
}

impl Parseable<Dispatch> for Dispatch {}
impl Parseable<Vec<Dispatch>> for Vec<Dispatch> {}

/// Represents a news article from Steam's news feed.
#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
pub struct SteamNews {
    /// The identifier assigned by Steam to this news item.
    pub id: String,
    /// The title of the Steam news item.
    pub title: String,
    /// The URL to the Steam news item.
    pub url: String,
    /// The author of the news item.
    pub author: String,
    /// The message posted formatted in Steam Markdown Format.
    pub content: String,
    /// When the message was posted.
    #[serde(rename = "publishedAt")]
    #[serde_as(as = "DateTime<Utc>")]
    pub published: NaiveDateTime,
}

impl Parseable<SteamNews> for SteamNews {}
impl Parseable<Vec<SteamNews>> for Vec<SteamNews> {}

impl HellApi {
    /// Retrieves a list of all available dispatch information.
    ///
    /// Endpoint: `/api/v1/dispatches`.
    pub async fn dispatches(&self) -> Result<Vec<Dispatch>> {
        self.request_blocking("/api/v1/dispatches").await
    }

    /// Retrieves a specific dispatch with identifier `id`.
    ///
    /// Endpoint: `/api/v1/dispatches/{id}`.
    pub async fn dispatch(&self, id: i32) -> Result<Dispatch> {
        let endpoint = format!("/api/v1/dispatches/{id}");
        self.request_blocking(endpoint.as_str()).await
    }

    /// Retrieves the Steam newsfeeed for Helldivers 2.
    ///
    /// Endpoint: `/api/v1/steam`.
    pub async fn steam_newsfeed(&self) -> Result<Vec<SteamNews>> {
        self.request_blocking("/api/v1/steam").await
    }

    /// Retrieves a specific newsfeed item from the Helldivers 2 Steam newsfeed.
    ///
    /// Endpoint: `/api/v1/steam/{gid}`.
    pub async fn steam_newsitem(&self, gid: &String) -> Result<SteamNews> {
        let endpoint = format!("/api/v1/steam/{gid}");
        self.request_blocking(endpoint.as_str()).await
    }
}

#[cfg(test)]
mod testing {
    use crate::prelude::testing::HELL_API_TEST;

    #[tokio::test]
    async fn dispatches_endpoint() {
        let result = HELL_API_TEST.dispatches().await;
        let inner = result.unwrap();
        let first = inner.first().unwrap();

        let result = HELL_API_TEST.dispatch(first.id).await;
        result.unwrap();
    }

    #[tokio::test]
    async fn steam_endpoint() {
        let result = HELL_API_TEST.steam_newsfeed().await;
        let inner = result.unwrap();
        let first = inner.first().unwrap();

        let result = HELL_API_TEST.steam_newsitem(&first.id).await;
        result.unwrap();
    }
}
