use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename = "feed")]
pub struct Feed {
    pub id: String,

    #[serde(rename = "channelId", alias = "yt:channelId")]
    pub yt_channel_id: String,
    pub title: String,

    pub published: String,
    #[serde(default)]
    pub entry: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub name: String,
    pub uri: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub id: String,
    pub title: String,

    pub author: Author,
    pub published: String,
    pub updated: String,

    #[serde(rename = "group", alias = "media:group")]
    pub media_group: MediaGroup,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaGroup {
    #[serde(rename = "thumbnail", alias = "media:thumbnail")]
    pub media_thumbnail: MediaThumbnail,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaThumbnail {
    pub url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}
