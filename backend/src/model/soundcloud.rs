use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};

const RSS_URL: &str = "https://feeds.soundcloud.com/users/soundcloud:users:$$ID$$/sounds.rss";

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SoundcloudFeed {
    #[serde(rename = "channel")]
    pub channel: Channel,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Channel {
    #[serde(rename = "title")]
    pub username: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "image")]
    pub profile_pictures: Vec<ImageElement>,

    #[serde(rename = "item")]
    pub item: Vec<Song>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Song {
    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "pubDate")]
    pub pub_date: String,

    #[serde(rename = "link")]
    pub link: String,

    #[serde(rename = "duration")]
    pub duration: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "enclosure")]
    pub enclosure: DataEnclosure,

    #[serde(rename = "image")]
    pub image: ImageElement,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DataEnclosure {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub data_enclosure_type: Option<Type>,

    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ImageElement {
    #[serde(rename = "href", default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "audio/mpeg")]
    #[default]
    AudioMpeg,
}

impl SoundcloudFeed {
    pub async fn get_content_from_id(id: &str, client: &Client) -> Result<SoundcloudFeed> {
        let url = RSS_URL.replace("$$ID$$", id);
        let response = client.get(&url).send().await?.text().await?;
        serde_xml_rs::from_str::<SoundcloudFeed>(&response).map_err(|_| anyhow!("xml skill issue"))
    }
}
