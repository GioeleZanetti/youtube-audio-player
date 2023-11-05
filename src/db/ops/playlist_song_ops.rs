use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};

use crate::db::{
    models::{NewPlaylistSong, PlaylistSong},
    schema::playlist_song::dsl::playlist_song,
};

pub fn add_songs_to_playlist(connection: &mut SqliteConnection, song: NewPlaylistSong) -> bool {
    diesel::insert_into(playlist_song)
        .values(song)
        .execute(connection)
        .is_ok()
}

pub fn get_songs_of_playlist(
    connection: &mut SqliteConnection,
    playlist_name: &str,
) -> Option<Vec<PlaylistSong>> {
    use crate::db::schema::playlist_song::dsl::playlist_name as name;
    playlist_song
        .filter(name.eq(playlist_name))
        .load(connection)
        .ok()
}

pub fn delete_playlist_song(
    connection: &mut SqliteConnection,
    playlist_song_to_delete: PlaylistSong,
) -> bool {
    use crate::db::schema::playlist_song::dsl::{playlist_name as name, song_id as song};

    diesel::delete(playlist_song)
        .filter(name.eq(playlist_song_to_delete.playlist_id))
        .filter(song.eq(playlist_song_to_delete.song_id))
        .execute(connection)
        .is_ok()
}
