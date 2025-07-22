use actix_web::HttpServer;
use actix_web::{App, HttpResponse, get, http::header::ContentType};
use model::youtube::Feed;
use reqwest::Client;

mod model;

const PORT: u16 = 9999;

const VENOX_ACCOUNT_IDS: [&str; 4] = [
    "UCs9v8InFppRaPtTJjOpxWWg",
    "UC3Olgcd6HHw1XSfnqKAqm9g",
    "UCh8XttfpNZxg2Q27iZ8DXcg",
    "UClhDo4tjwvbJLQYm71TF_Ag",
];

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on port {PORT}");
    HttpServer::new(|| App::new().service(venox_accounts))
        .bind(("127.0.0.1", PORT))?
        .run()
        .await
}

#[get("/venox_channels")]
async fn venox_accounts() -> HttpResponse {
    let client = Client::new();
    let mut feeds: Vec<Feed> = vec![];
    for account_id in VENOX_ACCOUNT_IDS {
        if let Ok(response) = Feed::get_content_from_id(account_id.to_string(), &client).await {
            feeds.push(response);
        }
    }
    let json_feeds = serde_json::to_string::<Vec<Feed>>(&feeds).expect("deserialization moment");
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(format!("{json_feeds}"))
}
