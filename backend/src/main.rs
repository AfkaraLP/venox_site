use anyhow::Result;
use model::youtube::Feed;
use reqwest::Client;

mod model;

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
