use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::db::models::{NewSong, Song};
use crate::db::schema::song::dsl::song;

pub fn get_songs(connection: &mut SqliteConnection) -> Option<Vec<Song>> {
    song.load(connection).ok()
}

pub fn insert_song(connection: &mut SqliteConnection, new_song: NewSong) -> bool {
    diesel::insert_into(song)
        .values(new_song)
        .execute(connection)
        .is_ok()
}

pub fn get_song_by_name(connection: &mut SqliteConnection, song_name: &str) -> Option<Song> {
    use crate::db::schema::song::dsl::name;
    song.filter(name.eq(song_name))
        .first::<Song>(connection)
        .ok()
}

pub fn get_song_by_id(connection: &mut SqliteConnection, song_id: &str) -> Option<Song> {
    use crate::db::schema::song::dsl::id;
    song.filter(id.eq(song_id)).first::<Song>(connection).ok()
}

pub fn delete_song(connection: &mut SqliteConnection, song_id: &str) -> bool {
    use crate::db::schema::song::dsl::id;

    let _activated_foreign_key = diesel::sql_query("PRAGMA foreign_keys = ON;").execute(connection);
    diesel::delete(song)
        .filter(id.eq(song_id))
        .execute(connection)
        .is_ok()
}
