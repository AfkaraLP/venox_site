use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};

const RSS_URL: &str = "https://feeds.soundcloud.com/users/soundcloud:users:$$ID$$/sounds.rss";

#[derive(Debug, Serialize, Deserialize)]
pub struct SoundcloudFeed {
    #[serde(rename = "channel")]
    channel: Channel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    #[serde(rename = "title")]
    username: String,

    #[serde(rename = "description")]
    description: String,

    #[serde(rename = "image")]
    profile_pictures: Vec<ImageElement>,

    #[serde(rename = "item")]
    item: Vec<Song>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Song {
    #[serde(rename = "title")]
    title: String,

    #[serde(rename = "pubDate")]
    pub_date: String,

    #[serde(rename = "link")]
    link: String,

    #[serde(rename = "duration")]
    duration: String,

    #[serde(rename = "description")]
    description: String,

    #[serde(rename = "enclosure")]
    enclosure: DataEnclosure,

    #[serde(rename = "image")]
    image: ImageElement,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataEnclosure {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    data_enclosure_type: Option<Type>,

    #[serde(rename = "url")]
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageElement {
    #[serde(rename = "href", default, skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "audio/mpeg")]
    AudioMpeg,
}

impl SoundcloudFeed {
    pub async fn get_content_from_id(id: String, client: &Client) -> Result<SoundcloudFeed> {
        let url = RSS_URL.replace("$$ID$$", id.as_str());
        let response = client.get(&url).send().await?.text().await?;
        serde_xml_rs::from_str::<SoundcloudFeed>(&response).map_err(|_| anyhow!("xml skill issue"))
    }
}
