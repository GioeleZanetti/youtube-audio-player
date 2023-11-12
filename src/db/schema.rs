// @generated automatically by Diesel CLI.

diesel::table! {
    playlist (name) {
        name -> Text,
    }
}

diesel::table! {
    playlist_song (playlist_name, song_id) {
        playlist_name -> Text,
        song_id -> Text,
    }
}

diesel::table! {
    song (id) {
        id -> Text,
        name -> Text,
        artist -> Nullable<Text>,
    }
}

diesel::joinable!(playlist_song -> playlist (playlist_name));
diesel::joinable!(playlist_song -> song (song_id));

diesel::allow_tables_to_appear_in_same_query!(playlist, playlist_song, song,);
