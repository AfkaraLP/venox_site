use actix_web::HttpServer;
use actix_web::{App, HttpResponse, get, http::header::ContentType};
use model::soundcloud::SoundcloudFeed;
use model::youtube::YoutubeFeed;
use reqwest::Client;

mod model;
mod util;

const PORT: u16 = 9999;

const VENOX_ACCOUNT_IDS: [&str; 4] = [
    "UCs9v8InFppRaPtTJjOpxWWg",
    "UC3Olgcd6HHw1XSfnqKAqm9g",
    "UCh8XttfpNZxg2Q27iZ8DXcg",
    "UClhDo4tjwvbJLQYm71TF_Ag",
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
async fn venox_accounts() -> HttpResponse {
    let client = Client::new();
    let mut feeds: Vec<YoutubeFeed> = vec![];
    for account_id in VENOX_ACCOUNT_IDS {
        if let Ok(response) =
            YoutubeFeed::get_content_from_id(account_id.to_string(), &client).await
        {
            feeds.push(response);
        }
    }
    let json_feeds =
        serde_json::to_string::<Vec<YoutubeFeed>>(&feeds).expect("deserialization moment");
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(format!("{json_feeds}"))
}

#[get("/soundcloud_data")]
async fn soundcloud_data() -> HttpResponse {
    let client = Client::new();
    if let Ok(response) =
        SoundcloudFeed::get_content_from_id(VENOX_SOUNDCLOUD_ID.to_string(), &client).await
    {
        let response = serde_json::to_string::<SoundcloudFeed>(&response).expect("deserilelizel");
        let response = HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(format!("{response}"));
        return response;
    }
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body("\"error\": \"getting sc data\"")
}
