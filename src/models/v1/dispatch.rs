use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;

use crate::{
    middleware,
    prelude::{Parseable, Result},
    HellApi,
};

/// A message than can either be a simple String or a LocalizedMessage.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Message {
    Simple(String),
    Localized(LocalizedMessage),
}

impl From<&str> for Message {
    fn from(value: &str) -> Self {
        Message::Simple(String::from(value))
    }
}

/// A localized message for a specific language.
#[non_exhaustive]
#[derive(Debug, Deserialize, PartialEq, Eq)]
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
#[derive(Debug, Deserialize, PartialEq, Eq)]
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

impl Parseable for Dispatch {}
impl Parseable for Vec<Dispatch> {}

/// Represents a news article from Steam's news feed.
#[non_exhaustive]
#[serde_with::serde_as]
#[derive(Debug, Deserialize, PartialEq, Eq)]
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

impl Parseable for SteamNews {}
impl Parseable for Vec<SteamNews> {}

impl HellApi {
    /// Retrieves a list of all available dispatch information.
    ///
    /// Endpoint: `/api/v1/dispatches`.
    pub async fn dispatches() -> Result<Vec<Dispatch>> {
        middleware::request_blocking("/api/v1/dispatches").await
    }

    /// Retrieves a specific dispatch with identifier `id`.
    ///
    /// Endpoint: `/api/v1/dispatches/{id}`.
    pub async fn dispatch(id: i32) -> Result<Dispatch> {
        let endpoint = format!("/api/v1/dispatches/{id}");
        middleware::request_blocking(endpoint.as_str()).await
    }

    /// Retrieves the Steam newsfeeed for Helldivers 2.
    ///
    /// Endpoint: `/api/v1/steam`.
    pub async fn steam_newsfeed() -> Result<Vec<SteamNews>> {
        middleware::request_blocking("/api/v1/steam").await
    }

    /// Retrieves a specific newsfeed item from the Helldivers 2 Steam newsfeed.
    ///
    /// Endpoint: `/api/v1/steam/{gid}`.
    pub async fn steam_newsitem(gid: &String) -> Result<SteamNews> {
        let endpoint = format!("/api/v1/steam/{gid}");
        middleware::request_blocking(endpoint.as_str()).await
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;

    use crate::{
        models::v1::dispatch::Message,
        prelude::{Parseable, TestValue},
    };

    use super::{Dispatch, SteamNews};

    impl TestValue for Dispatch {
        fn test_expected() -> Self {
            Dispatch {
                id: 0,
                published: NaiveDateTime::parse_from_str(
                    "2024-07-06T20:18:00.090Z",
                    "%Y-%m-%dT%H:%M:%S%.fZ",
                )
                .unwrap(),
                _type: 1,
                message: Message::from("string"),
            }
        }

        const TEST_JSON: &'static str = r#"
          {
            "id": 0,
            "published": "2024-07-06T20:18:00.090Z",
            "type": 1,
            "message": "string"
          }
        "#;
    }

    impl TestValue for SteamNews {
        fn test_expected() -> Self {
            SteamNews {
                id: String::from("0"),
                title: String::from("title"),
                url: String::from("url"),
                author: String::from("author"),
                content: String::from("content"),
                published: NaiveDateTime::parse_from_str(
                    "2024-07-06T20:18:00.090Z",
                    "%Y-%m-%dT%H:%M:%S%.fZ",
                )
                .unwrap(),
            }
        }

        const TEST_JSON: &'static str = r#"{
            "id": "0",
            "title": "title",
            "url": "url",
            "author": "author",
            "content": "content",
            "publishedAt": "2024-07-06T20:18:00.090Z"
        }"#;
    }

    #[test]
    fn parse_dispatch() {
        let json = serde_json::from_str(Dispatch::TEST_JSON).unwrap();
        let dispatch = Dispatch::parse(json).unwrap();

        assert_eq!(dispatch, Dispatch::test_expected());
    }

    #[test]
    fn parse_steam_news() {
        let json = serde_json::from_str(SteamNews::TEST_JSON).unwrap();
        let steam_news = SteamNews::parse(json).unwrap();

        assert_eq!(steam_news, SteamNews::test_expected());
    }
}
