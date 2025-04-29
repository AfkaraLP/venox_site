use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename = "feed")]
pub struct Feed {
    // #[serde(rename_all = "camelCase")]
    // pub link: Vec<Link>, // collects both self and alternate links
    pub id: String,

    #[serde(rename = "channelId", alias = "yt:channelId")]
    pub yt_channel_id: String, // tag is <yt:channelId>
    pub title: String,

    // pub author: Author,
    pub published: String,
    #[serde(default)]
    pub entry: Vec<Entry>,
}

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Link {
//     pub rel: Option<String>,
//     pub href: String,
// }

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

const RSS_URL: &str = "https://www.youtube.com/feeds/videos.xml?channel_id=";
const VENOX_ACCOUNT_IDS: [&str; 4] = [
    "UCs9v8InFppRaPtTJjOpxWWg",
    "UC3Olgcd6HHw1XSfnqKAqm9g",
    "UCh8XttfpNZxg2Q27iZ8DXcg",
    "UClhDo4tjwvbJLQYm71TF_Ag",
];
// this is a comment

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Client::new();

    let intm = client
        .get(format!("{SUMN}{VENOX_MINECRAFT}"))
        .send()
        .await?
        .text()
        .await?;

    let res: Feed = serde_xml_rs::from_str(&intm)?;

    println!("{:#?}", res);

    Ok(())
}
