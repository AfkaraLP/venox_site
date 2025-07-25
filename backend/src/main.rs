use actix_web::{App, HttpResponse, get, http::header::ContentType};
use actix_web::{HttpServer, Responder, web};
use model::soundcloud::SoundcloudFeed;
use model::youtube::{Author, Entry, MediaGroup, MediaThumbnail, YoutubeFeed};
use reqwest::Client;
use rusqlite::Connection;
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use util::db as venox_db;

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
async fn venox_accounts(data: web::Data<AppState>) -> impl Responder {
    let connection = data.connection.lock().await;
    let mut feeds: Vec<YoutubeFeed> = vec![];
    for id in VENOX_YT_ACCOUNT_IDS {
        let Some((_, id)) = id.split_once("UC") else {
            continue;
        };
        let mut statement = connection
            .prepare("SELECT yt_channel_id, title, published, id FROM youtube_feed WHERE yt_channel_id = (?1)")
            .unwrap();
        let mut rows = statement.query([id]).unwrap();
        if let Some(row) = rows.next().unwrap() {
            let mut feed = YoutubeFeed {
                id: row.get(3).unwrap(),
                yt_channel_id: row.get(0).unwrap(),
                title: row.get(1).unwrap(),
                published: row.get(2).unwrap(),
                entry: vec![],
            };
            let mut statement = connection
                .prepare(
                    "SELECT
                    id,
                    title,
                    author_name, 
                    author_uri,
                    published,
                    updated,
                    thumbnail_url,
                    thumbnail_width,
                    thumbnail_height
                FROM youtube_entry
                WHERE feed_id = (?1)
                ",
                )
                .unwrap();
            let entries = statement
                .query_map([&feed.id], |row| {
                    Ok(Entry {
                        id: row.get(0).unwrap(),
                        title: row.get(1).unwrap(),
                        author: Author {
                            name: row.get(2).unwrap(),
                            uri: row.get(3).unwrap(),
                        },
                        published: row.get(4).unwrap(),
                        updated: row.get(5).unwrap(),
                        media_group: MediaGroup {
                            media_thumbnail: MediaThumbnail {
                                url: row.get(6).unwrap(),
                                width: row.get(7).unwrap(),
                                height: row.get(8).unwrap(),
                            },
                        },
                    })
                })
                .unwrap();
            for entry in entries {
                feed.entry.push(entry.expect("y3s the entry indeed"));
            }
            feeds.push(feed);
        }
    }
    web::Json(feeds)
}

#[get("/soundcloud_data")]
async fn soundcloud_data(data: web::Data<AppState>) -> impl Responder {
    let connection = data.connection.lock().await;
    todo!();
    web::Json(json!({"error": "getting soundcloud data"}))
}
