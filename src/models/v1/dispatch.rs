use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;

use crate::{
    prelude::{Parseable, Result},
    HellApi,
};

/// A message than can either be a simple String or a LocalizedMessage.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Message {
    Simple(String),
    Localized(LocalizedMessage),
}

/// A localized message for a specific language.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct LocalizedMessage {
    /// The message in en-US.
    #[serde(rename = "en-US")]
    pub en_US: Option<String>,
    /// The message in de-DE.
    #[serde(rename = "de-DE")]
    pub de_DE: Option<String>,
    /// The message in es-ES.
    #[serde(rename = "es-ES")]
    pub es_ES: Option<String>,
    /// The message in ru-RU.
    #[serde(rename = "ru-RU")]
    pub ru_RU: Option<String>,
    /// The message in fr-FR.
    #[serde(rename = "fr-FR")]
    pub fr_FR: Option<String>,
    /// The message in it-IT.
    #[serde(rename = "it-IT")]
    pub it_IT: Option<String>,
    /// The message in pl-PL.
    #[serde(rename = "pl-PL")]
    pub pl_PL: Option<String>,
    /// The message in zh-Hans.
    #[serde(rename = "zh-Hans")]
    pub zh_Hans: Option<String>,
    /// The message in zh-Hant.
    #[serde(rename = "zh-Hant")]
    pub zh_Hant: Option<String>,
}

/// Represents a message from high command to the players like updates on the
/// status of the war effort.
#[non_exhaustive]
#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
pub struct Dispatch {
    /// The unique identifier of the dispatch.
    pub id: i32,
    /// The time when the dispatch was published.
    #[serde_as(as = "DateTime<Utc>")]
    pub published: NaiveDateTime,
    /// The type of dispatch. Purpose unknown as of now.
    #[serde(rename = "type")]
    pub _type: i32,
    /// The message that this dispatch represents.
    pub message: Message,
}

impl Parseable<Dispatch> for Dispatch {}
impl Parseable<Vec<Dispatch>> for Vec<Dispatch> {}

/// Represents a news article from Steam's news feed.
#[non_exhaustive]
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
    pub async fn dispatches() -> Result<Vec<Dispatch>> {
        Self::request_blocking("/api/v1/dispatches").await
    }

    /// Retrieves a specific dispatch with identifier `id`.
    ///
    /// Endpoint: `/api/v1/dispatches/{id}`.
    pub async fn dispatch(id: i32) -> Result<Dispatch> {
        let endpoint = format!("/api/v1/dispatches/{id}");
        Self::request_blocking(endpoint.as_str()).await
    }

    /// Retrieves the Steam newsfeeed for Helldivers 2.
    ///
    /// Endpoint: `/api/v1/steam`.
    pub async fn steam_newsfeed() -> Result<Vec<SteamNews>> {
        Self::request_blocking("/api/v1/steam").await
    }

    /// Retrieves a specific newsfeed item from the Helldivers 2 Steam newsfeed.
    ///
    /// Endpoint: `/api/v1/steam/{gid}`.
    pub async fn steam_newsitem(gid: &String) -> Result<SteamNews> {
        let endpoint = format!("/api/v1/steam/{gid}");
        Self::request_blocking(endpoint.as_str()).await
    }
}
