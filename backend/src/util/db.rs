use crate::model::soundcloud;
use crate::model::soundcloud::SoundcloudFeed;
use crate::model::youtube;
use crate::model::youtube::YoutubeFeed;
use anyhow::Result;
use rusqlite::{Connection, params};

pub fn initialize_soundcloud_db(connection: &Connection, feed: &SoundcloudFeed) -> Result<()> {
    let sql: &str = "
        -- Table for the feed's channel
        CREATE TABLE channels (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            description TEXT NOT NULL
        );

        -- Table for profile images of a channel (can have multiple)
        CREATE TABLE channel_images (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            channel_id INTEGER NOT NULL,
            url TEXT,
            FOREIGN KEY (channel_id) REFERENCES channels(id)
        );

        -- Table for songs/items
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
    ";

    connection.execute_batch(&sql)?;

    insert_soundcloud_feed(connection, feed)
}

pub fn insert_soundcloud_feed(connection: &Connection, feed: &SoundcloudFeed) -> Result<()> {
    connection.execute(
        "INSERT INTO channels (username, description) VALUES (?1, ?2)",
        params![feed.channel.username, feed.channel.description],
    )?;

    let channel_id = connection.last_insert_rowid();

    for image in &feed.channel.profile_pictures {
        connection.execute(
            "INSERT INTO channel_images (channel_id, url) VALUES (?1, ?2)",
            params![channel_id, image.url],
        )?;
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
}
