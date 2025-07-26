use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};

const RSS_URL: &str = "https://www.youtube.com/feeds/videos.xml?channel_id=";

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "feed")]
pub struct YoutubeFeed {
    pub id: String,

    #[serde(rename = "channelId", alias = "yt:channelId")]
    pub yt_channel_id: String,
    pub title: String,

    pub published: String,
    #[serde(default)]
    pub entry: Vec<Entry>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub name: String,
    pub uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaGroup {
    #[serde(rename = "thumbnail", alias = "media:thumbnail")]
    pub media_thumbnail: MediaThumbnail,

    #[serde(rename = "community", alias = "media:community")]
    pub media_community: MediaCommunity,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MediaCommunity {
    #[serde(rename = "starRating", alias = "media:starRating")]
    pub star_rating: StarRating,

    #[serde(rename = "statistics", alias = "media:statistics")]
    pub statistics: Statistics,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct StarRating {
    pub count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Statistics {
    pub views: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaThumbnail {
    pub url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl YoutubeFeed {
    pub async fn get_content_from_id(id: &str, client: &Client) -> Result<YoutubeFeed> {
        let response = client
            .get(format!("{RSS_URL}{id}"))
            .send()
            .await?
            .text()
            .await?;

        serde_xml_rs::from_str(&response).map_err(|_| anyhow!("serde xml issue"))
    }
}
