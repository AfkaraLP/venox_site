use actix_web::error::ErrorInternalServerError;
use actix_web::{App, Error, get};
use actix_web::{HttpServer, Responder, web};
use anyhow::Result;
use model::soundcloud::SoundcloudFeed;
use model::youtube::YoutubeFeed;
use reqwest::Client;
use rusqlite::Connection;
use std::ops::Index;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::sync::RwLock;
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
    sc_data: Arc<RwLock<Option<SoundcloudFeed>>>,
    yt_data: Arc<RwLock<Vec<YoutubeFeed>>>,
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
        sc_data: Arc::new(RwLock::new(None)),
        yt_data: Arc::new(RwLock::new(Vec::new())),
    };

    let mut store_interval = tokio::time::interval(Duration::from_secs(60));

    let connection = data.connection.clone();
    let mut sc_data = data.sc_data.clone();
    let mut yt_data = data.yt_data.clone();
    tokio::spawn(async move {
        loop {
            store_interval.tick().await;
            println!("fetched new database data");

            for (idx, account_id) in VENOX_YT_ACCOUNT_IDS.iter().enumerate() {
                if let Ok(response) = YoutubeFeed::get_content_from_id(account_id, &client).await {
                    let connection = connection.lock().await;
                    venox_db::insert_youtube_feed(&connection, &response)
                        .expect("insert new data into youtube feed");
                    let mut yt_data = yt_data.write().await;
                    yt_data.insert(idx, response);
                }
            }

            if let Ok(response) =
                SoundcloudFeed::get_content_from_id(VENOX_SOUNDCLOUD_ID, &client).await
            {
                let connection = connection.lock().await;
                venox_db::insert_soundcloud_feed(&connection, &response)
                    .expect("insert new data into soundcloud database");
                let mut sc_data = sc_data.write().await;
                sc_data.replace(response);
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
    if data.yt_data.read().await.len() > 0 {
        let feeds = data.yt_data.read().await;
        let feeds = serde_json::to_value(&*feeds)?;
        return Ok(web::Json(feeds));
    }
    let connection = data.connection.lock().await;
    let feeds =
        get_youtube_feeds_from_db(&connection, VENOX_YT_ACCOUNT_IDS.into()).map_err(|e| {
            eprintln!("Error getting youtube feeds: {e:?}");
            ErrorInternalServerError("Error Getting Youtube Feeds")
        })?;
    let feeds = serde_json::to_value(&*feeds)?;
    Ok(web::Json(feeds))
}

#[get("/soundcloud_data")]
async fn soundcloud_data(data: web::Data<AppState>) -> Result<impl Responder, Error> {
    if data.sc_data.read().await.is_some() {
        let sc_data = data.sc_data.write().await;
        let sc_data = serde_json::to_value(&*sc_data)?;
        return Ok(web::Json(sc_data));
    }

    let connection = data.connection.lock().await;
    let feeds = venox_db::get_soundcloud_feeds_from_db(&connection, vec![VENOX_SOUNDCLOUD_ID])
        .map_err(|e| {
            eprintln!("Error getting youtube feeds: {e:?}");
            ErrorInternalServerError("Error Getting Youtube Feeds")
        })?;
    let feed = feeds.into_iter().next();
    let feed = serde_json::to_value(feed)?;
    Ok(web::Json(feed))
}
