use crate::model::soundcloud::SoundcloudFeed;
use crate::model::youtube::{self, Author, MediaGroup};
use crate::model::youtube::{Entry, YoutubeFeed};
use crate::model::{soundcloud, youtube::MediaThumbnail};
use anyhow::Result;
use rusqlite::{Connection, params};

pub fn initialize_youtube_db(connection: &Connection) -> Result<()> {
    let sql: &str = "
        -- Main feed table
        CREATE TABLE IF NOT EXISTS youtube_feed (
            id TEXT PRIMARY KEY,
            yt_channel_id TEXT NOT NULL,
            title TEXT NOT NULL,
            published TEXT NOT NULL
        );

        -- Entries table, linked to youtube_feed
        CREATE TABLE IF NOT EXISTS youtube_entry (
            id TEXT PRIMARY KEY,
            feed_id TEXT NOT NULL,
            title TEXT NOT NULL,
            author_name TEXT NOT NULL,
            author_uri TEXT NOT NULL,
            published TEXT NOT NULL,
            updated TEXT NOT NULL,
            thumbnail_url TEXT NOT NULL,
            thumbnail_width INTEGER,
            thumbnail_height INTEGER,
            FOREIGN KEY(feed_id) REFERENCES youtube_feed(id) ON DELETE CASCADE
        );
    ";

    connection.execute_batch(sql)?;
    Ok(())
}

pub fn insert_youtube_feed(connection: &Connection, feed: &YoutubeFeed) -> Result<()> {
    // Insert into youtube_feed
    connection.execute(
        "INSERT OR REPLACE INTO youtube_feed (id, yt_channel_id, title, published)
         VALUES (?1, ?2, ?3, ?4)",
        params![feed.id, feed.yt_channel_id, feed.title, feed.published],
    )?;

    // Insert each entry
    for entry in &feed.entry {
        connection.execute(
            "INSERT OR REPLACE INTO youtube_entry (
                id, feed_id, title,
                author_name, author_uri,
                published, updated,
                thumbnail_url, thumbnail_width, thumbnail_height
             )
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                entry.id,
                feed.id,
                entry.title,
                entry.author.name,
                entry.author.uri,
                entry.published,
                entry.updated,
                entry.media_group.media_thumbnail.url,
                entry.media_group.media_thumbnail.width,
                entry.media_group.media_thumbnail.height,
            ],
        )?;
    }

    Ok(())
}

pub fn initialize_soundcloud_db(connection: &Connection) -> Result<()> {
    let sql: &str = "
        -- Table for the feed's channel
        CREATE TABLE IF NOT EXISTS channels (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            description TEXT NOT NULL
        );

        -- Table for profile images of a channel (can have multiple)
        CREATE TABLE IF NOT EXISTS channel_images (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            channel_id INTEGER NOT NULL,
            url TEXT,
            FOREIGN KEY (channel_id) REFERENCES channels(id)
        );

        -- Table for songs/items
        CREATE TABLE IF NOT EXISTS songs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            channel_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            pub_date TEXT NOT NULL,
            link TEXT NOT NULL,
            duration TEXT NOT NULL,
            description TEXT NOT NULL,
            enclosure_url TEXT NOT NULL,
            enclosure_type TEXT,
            image_url TEXT,
            FOREIGN KEY (channel_id) REFERENCES channels(id)
        );
    ";

    connection.execute_batch(&sql)?;

    Ok(())
}

pub fn insert_soundcloud_feed(connection: &Connection, feed: &SoundcloudFeed) -> Result<()> {
    connection.execute(
        "INSERT INTO channels (username, description) VALUES (?1, ?2)",
        params![feed.channel.username, feed.channel.description],
    )?;

    let channel_id = connection.last_insert_rowid();

    for image in &feed.channel.profile_pictures {
        if let Some(url) = &image.url {
            connection.execute(
                "INSERT INTO channel_images (channel_id, url) VALUES (?1, ?2)",
                params![channel_id, url],
            )?;
        }
    }

    for song in &feed.channel.item {
        connection.execute(
            "INSERT INTO songs (
                channel_id, title, pub_date, link, duration, description,
                enclosure_url, enclosure_type, image_url
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                channel_id,
                song.title,
                song.pub_date,
                song.link,
                song.duration,
                song.description,
                song.enclosure.url,
                song.enclosure
                    .data_enclosure_type
                    .as_ref()
                    .map(|t| match t {
                        soundcloud::Type::AudioMpeg => "audio/mpeg",
                    }),
                song.image.url
            ],
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rusqlite::Connection;

    #[test]
    fn test_insert_soundcloud_feed() {
        let conn = Connection::open_in_memory().expect("in-memory db failed");

        // Create schema
        conn.execute_batch(
            r#"
            CREATE TABLE channels (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL,
                description TEXT NOT NULL
            );

            CREATE TABLE channel_images (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                channel_id INTEGER NOT NULL,
                url TEXT,
                FOREIGN KEY (channel_id) REFERENCES channels(id)
            );

            CREATE TABLE songs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                channel_id INTEGER NOT NULL,
                title TEXT NOT NULL,
                pub_date TEXT NOT NULL,
                link TEXT NOT NULL,
                duration TEXT NOT NULL,
                description TEXT NOT NULL,
                enclosure_url TEXT NOT NULL,
                enclosure_type TEXT,
                image_url TEXT,
                FOREIGN KEY (channel_id) REFERENCES channels(id)
            );
        "#,
        )
        .unwrap();

        // Build dummy feed
        let feed = SoundcloudFeed {
            channel: soundcloud::Channel {
                username: "test_user".to_string(),
                description: "test description".to_string(),
                profile_pictures: vec![soundcloud::ImageElement {
                    url: Some("http://example.com/image.jpg".to_string()),
                    ..Default::default()
                }],
                item: vec![soundcloud::Song {
                    title: "Test Song".to_string(),
                    pub_date: "Wed, 23 Jul 2025 00:00:00 GMT".to_string(),
                    link: "http://example.com/song".to_string(),
                    duration: "123".to_string(),
                    description: "A test song".to_string(),
                    enclosure: soundcloud::DataEnclosure {
                        url: "http://example.com/audio.mp3".to_string(),
                        data_enclosure_type: Some(soundcloud::Type::AudioMpeg),
                        ..Default::default()
                    },
                    image: soundcloud::ImageElement {
                        url: Some("http://example.com/song.jpg".to_string()),
                        ..Default::default()
                    },
                }],
            },
        };

        // Insert data
        insert_soundcloud_feed(&conn, &feed).expect("insert failed");

        // Assert channel inserted
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM channels WHERE username = 'test_user'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);

        // Assert song inserted
        let song_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM songs WHERE title = 'Test Song'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(song_count, 1);

        // Assert image inserted
        let img_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM channel_images WHERE url = 'http://example.com/image.jpg'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(img_count, 1);
    }

    fn setup_schema(conn: &Connection) {
        conn.execute_batch(
            "
            CREATE TABLE youtube_feed (
                id TEXT PRIMARY KEY,
                yt_channel_id TEXT NOT NULL,
                title TEXT NOT NULL,
                published TEXT NOT NULL
            );
            
            CREATE TABLE youtube_entry (
                id TEXT PRIMARY KEY,
                feed_id TEXT NOT NULL,
                title TEXT NOT NULL,
                author_name TEXT NOT NULL,
                author_uri TEXT NOT NULL,
                published TEXT NOT NULL,
                updated TEXT NOT NULL,
                thumbnail_url TEXT NOT NULL,
                thumbnail_width INTEGER,
                thumbnail_height INTEGER,
                FOREIGN KEY(feed_id) REFERENCES youtube_feed(id) ON DELETE CASCADE
            );
            ",
        )
        .unwrap();
    }

    #[test]
    fn test_insert_youtube_feed() {
        let conn = Connection::open_in_memory().unwrap();
        setup_schema(&conn);

        let feed = YoutubeFeed {
            id: "feed1".to_string(),
            yt_channel_id: "channel123".to_string(),
            title: "Sample Feed".to_string(),
            published: "2025-01-01T00:00:00Z".to_string(),
            entry: vec![youtube::Entry {
                id: "entry1".to_string(),
                title: "First Video".to_string(),
                author: youtube::Author {
                    name: "Uploader".to_string(),
                    uri: "https://youtube.com/uploader".to_string(),
                },
                published: "2025-01-01T01:00:00Z".to_string(),
                updated: "2025-01-01T02:00:00Z".to_string(),
                media_group: youtube::MediaGroup {
                    media_thumbnail: youtube::MediaThumbnail {
                        url: "https://img.youtube.com/vi/xyz/default.jpg".to_string(),
                        width: Some(120),
                        height: Some(90),
                    },
                },
            }],
        };

        insert_youtube_feed(&conn, &feed).unwrap();

        // Check feed was inserted
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM youtube_feed").unwrap();
        let count: i64 = stmt.query_row([], |row| row.get(0)).unwrap();
        assert_eq!(count, 1);

        // Check entry was inserted
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM youtube_entry").unwrap();
        let count: i64 = stmt.query_row([], |row| row.get(0)).unwrap();
        assert_eq!(count, 1);
    }
}

pub fn get_youtube_feeds_from_db(
    connection: &Connection,
    account_ids: Vec<&str>,
) -> Result<Vec<YoutubeFeed>> {
    let mut feeds: Vec<YoutubeFeed> = vec![];
    for id in account_ids {
        let Some((_, id)) = id.split_once("UC") else {
            continue;
        };
        let mut statement = connection.prepare(
            "SELECT
                yt_channel_id,
                title,
                published,
                id
            FROM
                youtube_feed
            WHERE
                yt_channel_id = (?1)",
        )?;
        let mut rows = statement.query([id])?;
        if let Some(row) = rows.next()? {
            let mut feed = YoutubeFeed {
                id: row.get(3)?,
                yt_channel_id: row.get(0)?,
                title: row.get(1)?,
                published: row.get(2)?,
                entry: vec![],
            };
            let mut statement = connection.prepare(
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
                    FROM
                        youtube_entry
                    WHERE
                        feed_id = (?1)
                ",
            )?;
            let entries = statement.query_map([&feed.id], |row| {
                Ok(Entry {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    author: Author {
                        name: row.get(2)?,
                        uri: row.get(3)?,
                    },
                    published: row.get(4)?,
                    updated: row.get(5)?,
                    media_group: MediaGroup {
                        media_thumbnail: MediaThumbnail {
                            url: row.get(6)?,
                            width: row.get(7)?,
                            height: row.get(8)?,
                        },
                    },
                })
            })?;
            for entry in entries {
                feed.entry.push(entry?);
            }
            feeds.push(feed);
        }
    }
    Ok(feeds)
}

pub fn get_soundcloud_feeds_from_db(
    connection: &Connection,
    account_ids: Vec<&str>,
) -> Result<Vec<SoundcloudFeed>> {
    let mut feeds: Vec<SoundcloudFeed> = vec![];
    let mut statement = connection.prepare(
        "
            SELECT
                username,
                description
            FROM
                channels
            WHERE
                id = (?1)
        ",
    )?;
    for id in account_ids {
        let mut rows = statement.query([id])?;
    }
    //     -- Table for the feed's channel
    //     CREATE TABLE IF NOT EXISTS channels (
    //         id INTEGER PRIMARY KEY AUTOINCREMENT,
    //         username TEXT NOT NULL,
    //         description TEXT NOT NULL
    //     );

    //     -- Table for profile images of a channel (can have multiple)
    //     CREATE TABLE IF NOT EXISTS channel_images (
    //         id INTEGER PRIMARY KEY AUTOINCREMENT,
    //         channel_id INTEGER NOT NULL,
    //         url TEXT,
    //         FOREIGN KEY (channel_id) REFERENCES channels(id)
    //     );

    //     -- Table for songs/items
    //     CREATE TABLE IF NOT EXISTS songs (
    //         id INTEGER PRIMARY KEY AUTOINCREMENT,
    //         channel_id INTEGER NOT NULL,
    //         title TEXT NOT NULL,
    //         pub_date TEXT NOT NULL,
    //         link TEXT NOT NULL,
    //         duration TEXT NOT NULL,
    //         description TEXT NOT NULL,
    //         enclosure_url TEXT NOT NULL,
    //         enclosure_type TEXT,
    //         image_url TEXT,
    //         FOREIGN KEY (channel_id) REFERENCES channels(id)
    //     );
    // ";
    Ok(feeds)
}
