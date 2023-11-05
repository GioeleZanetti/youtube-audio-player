// @generated automatically by Diesel CLI.

diesel::table! {
    playlist (name) {
        #[max_length = 200]
        name -> Varchar,
    }
}

diesel::table! {
    playlist_song (playlist_name, song_id) {
        #[max_length = 200]
        playlist_name -> Varchar,
        #[max_length = 50]
        song_id -> Varchar,
    }
}

diesel::table! {
    song (id) {
        #[max_length = 50]
        id -> Varchar,
        #[max_length = 200]
        name -> Varchar,
    }
}

diesel::joinable!(playlist_song -> playlist (playlist_name));
diesel::joinable!(playlist_song -> song (song_id));

diesel::allow_tables_to_appear_in_same_query!(
    playlist,
    playlist_song,
    song,
);
