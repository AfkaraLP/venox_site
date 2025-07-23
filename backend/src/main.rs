use actix_web::{App, HttpResponse, get, http::header::ContentType};
use actix_web::{HttpServer, Responder, web};
use model::soundcloud::SoundcloudFeed;
use model::youtube::YoutubeFeed;
use reqwest::Client;
use serde_json::json;

mod model;
mod util;

const PORT: u16 = 9999;

const VENOX_ACCOUNT_IDS: [&str; 5] = [
    "UCs9v8InFppRaPtTJjOpxWWg",
    "UC3Olgcd6HHw1XSfnqKAqm9g",
    "UCh8XttfpNZxg2Q27iZ8DXcg",
    "UClhDo4tjwvbJLQYm71TF_Ag",
    "UCYPixV_-08kSKkVFWyidbFw",
];

const VENOX_SOUNDCLOUD_ID: &str = "001310885850";
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on port {PORT}");
    HttpServer::new(|| App::new().service(venox_accounts).service(soundcloud_data))
        .bind(("0.0.0.0", PORT))?
        .run()
        .await
}

#[get("/youtube_videos")]
async fn venox_accounts() -> impl Responder {
    let client = Client::new();
    let mut feeds: Vec<YoutubeFeed> = vec![];
    for account_id in VENOX_ACCOUNT_IDS {
        if let Ok(response) =
            YoutubeFeed::get_content_from_id(account_id.to_string(), &client).await
        {
            feeds.push(response);
        }
    }
    // let json_feeds =
    //     serde_json::to_string::<Vec<YoutubeFeed>>(&feeds).expect("deserialization moment");
    web::Json(feeds)
}

#[get("/soundcloud_data")]
async fn soundcloud_data() -> impl Responder {
    let client = Client::new();
    if let Ok(response) =
        SoundcloudFeed::get_content_from_id(VENOX_SOUNDCLOUD_ID.to_string(), &client).await
    {
        return web::Json(json!(response));
    }
    web::Json(json!({"error": "getting soundcloud data"}))
}
