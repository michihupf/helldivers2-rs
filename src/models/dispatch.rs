use chrono::{DateTime, Utc};

/// A message than can either be a simple String or a LocalizedMessage.
pub enum Message {
    Simple(String),
    Localized(LocalizedMessage),
}

/// A localized message for a specific language.
#[allow(non_snake_case)]
pub struct LocalizedMessage {
    /// The message in en-US.
    en_US: Option<String>,
    /// The message in de-DE.
    de_DE: Option<String>,
    /// The message in es-ES.
    es_ES: Option<String>,
    /// The message in ru-RU.
    ru_RU: Option<String>,
    /// The message in fr-FR.
    fr_FR: Option<String>,
    /// The message in it-IT.
    it_IT: Option<String>,
    /// The message in pl-PL.
    pl_PL: Option<String>,
    /// The message in zh-Hans.
    zh_Hans: Option<String>,
    /// The message in zh-Hant.
    zh_Hant: Option<String>,
}
/// Represents a message from high command to the players like updates on the

/// status of the war effort.
pub struct Dispatch {
    /// The unique identifier of the dispatch.
    id: i32,
    /// The time when the dispatch was published.
    published: DateTime<Utc>,
    /// The type of dispatch. Purpose unknown as of now.
    _type: i32,
    message: Message,
}

/// Represents an item in the newsfeed of Super Earth.
pub struct NewsFeedItem {
    /// The identifier of this newsfeed item.
    id: i32,
    /// The time when this item was published
    published: DateTime<Utc>,
    /// A numerical type. Purpose is unknown.
    _type: i32,
    /// The message.
    message: String,
}

/// Represents a news article from Steam's news feed.
pub struct SteamNews {
    /// The identifier assigned by Steam to this news item.
    id: String,
    /// The title of the Steam news item.
    title: String,
    /// The URL to the Steam news item.
    url: String,
    /// The author of the news item.
    author: String,
    /// The message posted formatted in Steam Markdown Format.
    content: String,
    /// When the message was posted.
    published: DateTime<Utc>,
}
