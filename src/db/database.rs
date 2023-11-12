use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    SqliteConnection,
};
use diesel_migrations::*;

use super::{
    models::{NewPlaylist, NewPlaylistSong, NewSong, Playlist, PlaylistSong, Song},
    ops::{
        playlist_ops::{delete_playlist, get_playlist_by_name, get_playlists, insert_playlist},
        playlist_song_ops::{add_songs_to_playlist, delete_playlist_song, get_songs_of_playlist},
        song_ops::{delete_song, get_song_by_id, get_song_by_name, get_songs, insert_song},
    },
};

type MysqlitePool = Pool<ConnectionManager<SqliteConnection>>;

/**
 * Struct di gestione del database
 */
pub struct Database {
    //Il pool di connessioni al database
    pool: MysqlitePool,
}

impl Database {
    /**
     * Costruttore dello struct, crea un istanza del dataabse
     * url: l'url per collegarsi al database
     * max_size: quante connessioni possono avvenire simultaneamente
     */
    pub fn new(url: String, max_size: u32) -> anyhow::Result<Self> {
        let pool = MysqlitePool::builder()
            .max_size(max_size)
            .build(ConnectionManager::new(url))?;

        Ok(Self { pool })
    }

    /**
     * Metodo per ottenere una connessione al database
     */
    fn get_connection(&self) -> PooledConnection<ConnectionManager<diesel::SqliteConnection>> {
        self.pool.get().unwrap()
    }

    /**
     * Metodo per eseguire le migrazioni se queste non sono ancora state eseguite
     */
    pub fn run_embedded_migrations(&self) -> bool {
        const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
        self.get_connection()
            .run_pending_migrations(MIGRATIONS)
            .is_ok()
    }

    pub fn get_songs(&self) -> Option<Vec<Song>> {
        get_songs(&mut self.get_connection())
    }

    pub fn get_song_by_name(&self, song_name: &str) -> Option<Song> {
        get_song_by_name(&mut self.get_connection(), song_name)
    }

    pub fn get_song_by_id(&self, song_id: &str) -> Option<Song> {
        get_song_by_id(&mut self.get_connection(), song_id)
    }

    pub fn insert_song(&self, new_song: NewSong) -> bool {
        insert_song(&mut self.get_connection(), new_song)
    }

    pub fn delete_song(&self, song_id: &str) -> bool {
        delete_song(&mut self.get_connection(), song_id)
    }

    pub fn get_playlists(&self) -> Option<Vec<Playlist>> {
        get_playlists(&mut self.get_connection())
    }

    pub fn insert_playlist(&self, new_playlist: NewPlaylist) -> bool {
        insert_playlist(&mut self.get_connection(), new_playlist)
    }

    pub fn delete_playlist(&self, playlist_name: &str) -> bool {
        delete_playlist(&mut self.get_connection(), playlist_name)
    }

    pub fn add_songs_to_playlist(&self, song: NewPlaylistSong) -> bool {
        add_songs_to_playlist(&mut self.get_connection(), song)
    }

    pub fn get_songs_of_playlist(&self, playlist_name: &str) -> Option<Vec<PlaylistSong>> {
        get_songs_of_playlist(&mut self.get_connection(), playlist_name)
    }
    pub fn get_playlist_by_name(&self, playlist_name: &str) -> Option<Playlist> {
        get_playlist_by_name(&mut self.get_connection(), playlist_name)
    }

    pub fn delete_playlist_song(&self, playlist_song: PlaylistSong) -> bool {
        delete_playlist_song(&mut self.get_connection(), playlist_song)
    }
}
