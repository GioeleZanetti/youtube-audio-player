use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};

use crate::db::{
    models::{NewPlaylist, Playlist},
    schema::playlist::dsl::playlist,
};

pub fn get_playlists(connnection: &mut SqliteConnection) -> Option<Vec<Playlist>> {
    playlist.load(connnection).ok()
}

pub fn insert_playlist(connnection: &mut SqliteConnection, new_playlist: NewPlaylist) -> bool {
    diesel::insert_into(playlist)
        .values(new_playlist)
        .execute(connnection)
        .is_ok()
}

pub fn delete_playlist(connection: &mut SqliteConnection, playlist_name: &str) -> bool {
    use crate::db::schema::playlist::dsl::name;

    let _activated_foreign_key = diesel::sql_query("PRAGMA foreign_keys = ON;").execute(connection);
    diesel::delete(playlist)
        .filter(name.eq(playlist_name))
        .execute(connection)
        .is_ok()
}

pub fn get_playlist_by_name(
    connnection: &mut SqliteConnection,
    playlist_name: &str,
) -> Option<Playlist> {
    use crate::db::schema::playlist::dsl::name;
    playlist
        .filter(name.eq(playlist_name))
        .first::<Playlist>(connnection)
        .ok()
}
