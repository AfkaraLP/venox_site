use actix_web::error::ErrorInternalServerError;
use actix_web::{App, Error, get};
use actix_web::{HttpServer, Responder, web};
use anyhow::Result;
use model::soundcloud::SoundcloudFeed;
use model::youtube::YoutubeFeed;
use reqwest::Client;
use rusqlite::Connection;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use util::db::{self as venox_db, get_youtube_feeds_from_db};

mod model;
mod util;

const PORT: u16 = 9999;

const VENOX_YT_ACCOUNT_IDS: [&str; 4] = [
    "UCs9v8InFppRaPtTJjOpxWWg",
    "UC3Olgcd6HHw1XSfnqKAqm9g",
    "UCh8XttfpNZxg2Q27iZ8DXcg",
    "UClhDo4tjwvbJLQYm71TF_Ag",
    // "UCYPixV_-08kSKkVFWyidbFw",
];

const VENOX_SOUNDCLOUD_ID: &str = "001310885850";

#[derive(Clone)]
struct AppState {
    connection: Arc<Mutex<Connection>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client: Client = Client::new();
    let connection: Connection =
        Connection::open("channels_info.db").expect("couldn't connect to database");
    venox_db::initialize_soundcloud_db(&connection).expect("initialize database");
    venox_db::initialize_youtube_db(&connection).expect("initialize database");
    let data = AppState {
        connection: Arc::new(Mutex::new(connection)),
    };

    let mut store_interval = tokio::time::interval(Duration::from_secs(60));

    let connection = data.connection.clone();
    tokio::spawn(async move {
        loop {
            store_interval.tick().await;
            println!("fetched new database data");

            for account_id in VENOX_YT_ACCOUNT_IDS {
                if let Ok(response) = YoutubeFeed::get_content_from_id(account_id, &client).await {
                    let connection = connection.lock().await;
                    venox_db::insert_youtube_feed(&connection, &response)
                        .expect("insert new data into youtube feed");
                }
            }

            if let Ok(response) =
                SoundcloudFeed::get_content_from_id(VENOX_SOUNDCLOUD_ID, &client).await
            {
                let connection = connection.lock().await;
                venox_db::insert_soundcloud_feed(&connection, &response)
                    .expect("insert new data into soundcloud database");
            }
        }
    });

    println!("opening server on port {PORT}");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data.clone()))
            .service(venox_accounts)
            .service(soundcloud_data)
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
}

#[get("/youtube_videos")]
async fn venox_accounts(data: web::Data<AppState>) -> Result<impl Responder, Error> {
    let connection = data.connection.lock().await;
    let feeds =
        get_youtube_feeds_from_db(&connection, VENOX_YT_ACCOUNT_IDS.into()).map_err(|e| {
            eprintln!("Error getting youtube feeds: {e:?}");
            ErrorInternalServerError("Error Getting Youtube Feeds")
        })?;

    Ok(web::Json(feeds))
}

#[get("/soundcloud_data")]
async fn soundcloud_data(data: web::Data<AppState>) -> Result<impl Responder, Error> {
    let connection = data.connection.lock().await;
    let feeds = venox_db::get_soundcloud_feeds_from_db(&connection, vec![VENOX_SOUNDCLOUD_ID])
        .map_err(|e| {
            eprintln!("Error getting youtube feeds: {e:?}");
            ErrorInternalServerError("Error Getting Youtube Feeds")
        })?;
    Ok(web::Json(feeds.into_iter().next()))
}
