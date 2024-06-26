use anyhow::anyhow;

use crate::{
    db::{
        database::Database,
        models::{NewPlaylist, NewPlaylistSong, NewSong, PlaylistSong},
    },
    mpd::mpd_client::MpdClient,
    youtube_api::youtube_api::YoutubeAPI,
};

pub struct Handler {
    database: Database,
    api: YoutubeAPI,
    mpd: MpdClient,
}

impl Handler {
    pub fn new(database: Database, api: YoutubeAPI, mpd: MpdClient) -> Self {
        database.run_embedded_migrations();
        Self { database, api, mpd }
    }

    pub async fn add_song_to_registry(
        &self,
        song_id: &str,
        song_name: &str,
        song_artist: Option<String>,
    ) -> anyhow::Result<()> {
        let inserted = self.database.insert_song(NewSong {
            id: song_id,
            name: song_name,
            artist: song_artist,
        });

        if !inserted {
            return Err(anyhow!("Couldn't add song to database, skipping..."));
        }

        self.api.download_audio(song_id, song_name).await?;
        println!("Song {} downloaded successfully", song_name);
        self.mpd.update_db()?;
        Ok(())
    }

    pub fn delete_song(&self, song_name: &str) -> anyhow::Result<()> {
        let song_id = match self.database.get_song_by_name(song_name) {
            Some(song) => song.id,
            None => {
                return Err(anyhow!(format!("Song {} doesn't exist", song_name)));
            }
        };
        self.api.delete_audio(&song_id)?;
        let deleted = self.database.delete_song(&song_id);
        if !deleted {
            return Err(anyhow!(format!(
                "Couldn't delete song {} from database",
                &song_name
            )));
        }
        println!("Song {} deleted successfully", song_name);
        self.mpd.update_db()?;
        Ok(())
    }

    pub fn create_playlist(
        &self,
        playlist_name: &str,
        songs_to_add: Vec<String>,
    ) -> anyhow::Result<()> {
        let inserted = self.database.insert_playlist(NewPlaylist {
            name: playlist_name,
        });

        if !inserted {
            return Err(anyhow!("Couldn't create playlist, skipping..."));
        }

        for song in songs_to_add {
            if let Some(song_in_db) = self.database.get_song_by_name(song.trim()) {
                self.database.add_songs_to_playlist(NewPlaylistSong {
                    playlist_name,
                    song_id: &song_in_db.id,
                });

                println!("Song {} added to playlist", &song);
            } else {
                return Err(anyhow!(format!("Song {} doesn't exist", &song)));
            }
        }

        Ok(())
    }

    pub fn delete_playlist(&self, playlist_name: &str) -> anyhow::Result<()> {
        let playlists = match self.database.get_playlists() {
            Some(playlist) => playlist,
            None => {
                return Err(anyhow!("No playlists in database"));
            }
        };

        if !playlists
            .iter()
            .map(|p| p.name.clone())
            .collect::<Vec<String>>()
            .contains(&playlist_name.to_string())
        {
            return Err(anyhow!(format!("Playlist {} doesn't exist", playlist_name)));
        }

        let deleted = self.database.delete_playlist(playlist_name);

        if !deleted {
            return Err(anyhow!(format!(
                "Couldn't delete playlist {}",
                playlist_name
            )));
        };

        println!("Playlist {} deleted successfully", playlist_name);
        Ok(())
    }

    pub fn play_playlist(&self, playlist_name: &str) -> anyhow::Result<()> {
        self.mpd.pause(Some(true))?;
        self.mpd.clear_queue()?;
        if self.database.get_playlist_by_name(playlist_name).is_none() {
            return Err(anyhow!(format!("Playlist {} doesn't exist", playlist_name)));
        }
        let songs = match self.database.get_songs_of_playlist(playlist_name) {
            Some(songs) => songs,
            None => {
                return Err(anyhow!("Playlist doesn't contain any songs"));
            }
        };
        for song in songs {
            self.mpd.add_to_queue(&song.song_id)?;
        }
        self.mpd.play()?;
        Ok(())
    }

    pub fn get_playlists(&self) -> anyhow::Result<()> {
        match self.database.get_playlists() {
            Some(playlists) => {
                for playlist in playlists {
                    println!("{}", playlist.name)
                }
            }
            None => {}
        };
        Ok(())
    }

    pub fn get_songs(&self) -> anyhow::Result<()> {
        match self.database.get_songs() {
            Some(songs) => {
                for song in songs {
                    println!("{} - {}", song.name, {
                        if let Some(artist) = song.artist {
                            artist
                        } else {
                            "Unknown".to_string()
                        }
                    })
                }
            }
            None => {}
        };
        Ok(())
    }

    pub fn play_song(&self, song_name: &str) -> anyhow::Result<()> {
        let song_id = match self.database.get_song_by_name(song_name) {
            Some(song) => song.id,
            None => {
                return Err(anyhow!(format!("Song {} doesn't exist", song_name)));
            }
        };
        self.mpd.pause(Some(true))?;
        self.mpd.clear_queue()?;
        self.mpd.add_to_queue(&song_id)?;
        self.mpd.play()?;
        Ok(())
    }

    pub fn play(&self) -> anyhow::Result<()> {
        self.mpd.play()?;
        Ok(())
    }

    pub fn pause(&self) -> anyhow::Result<()> {
        self.mpd.pause(None)?;
        Ok(())
    }

    pub fn shuffle(&self) -> anyhow::Result<()> {
        self.mpd.shuffle(None)?;
        Ok(())
    }

    pub fn clear_queue(&self) -> anyhow::Result<()> {
        self.mpd.clear_queue()?;
        println!("Queue cleared");
        Ok(())
    }

    pub fn next(&self) -> anyhow::Result<()> {
        self.mpd.next()?;
        self.mpd.pause(Some(true))?;
        self.mpd.pause(Some(false))?;
        println!("Skipping to next song in queue");
        Ok(())
    }

    pub fn previous(&self) -> anyhow::Result<()> {
        self.mpd.previous()?;
        self.mpd.pause(Some(true))?;
        self.mpd.pause(Some(false))?;
        println!("Going back to previous song in queue");
        Ok(())
    }

    pub fn insert_into_playlist(&self, playlist_name: &str, song_name: &str) -> anyhow::Result<()> {
        if self.database.get_playlist_by_name(playlist_name).is_none() {
            return Err(anyhow!(format!("Playlist {} doesn't exist", playlist_name)));
        };

        let song = match self.database.get_song_by_name(song_name) {
            Some(song) => song,
            None => {
                return Err(anyhow!(format!("Song {} doesn't exist", song_name)));
            }
        };

        let inserted = self.database.add_songs_to_playlist(NewPlaylistSong {
            playlist_name,
            song_id: &song.id,
        });

        if !inserted {
            return Err(anyhow!(
                "Couldn't insert song into playlist, song is already present"
            ));
        };

        println!("Song {} successfully added to {}", song_name, playlist_name);
        Ok(())
    }

    pub fn remove_song_from_playlist(
        &self,
        playlist_name: &str,
        song_name: &str,
    ) -> anyhow::Result<()> {
        let song = match self.database.get_song_by_name(song_name) {
            Some(song) => song,
            None => {
                return Err(anyhow!(format!("Song {} doesn't exist", song_name)));
            }
        };
        if self.database.get_playlist_by_name(playlist_name).is_none() {
            return Err(anyhow!(format!("Playlist {} doesn't exist", playlist_name)));
        }
        let removed = self.database.delete_playlist_song(PlaylistSong {
            playlist_id: playlist_name.to_string(),
            song_id: song.id,
        });
        if !removed {
            return Err(anyhow!("Couldn't remove song from playlist"));
        }
        Ok(())
    }

    pub fn current(&self) -> anyhow::Result<()> {
        let current_song = self.mpd.current()?;
        let current_time = self.mpd.current_time()?;
        let current_song_name = match current_song.split(".").next() {
            Some(name) => name,
            None => {
                return Err(anyhow!("Error in file name"));
            }
        };
        let song_info = match self.database.get_song_by_id(&current_song_name) {
            Some(song) => song,
            None => {
                return Err(anyhow!("No song currently playing"));
            }
        };

        println!(
            "Current song: {}\nArtist: {}\n{}",
            song_info.name,
            {
                if let Some(song_artist) = song_info.artist {
                    song_artist
                } else {
                    "Unknown".to_string()
                }
            },
            current_time
        );

        Ok(())
    }

    pub fn repeat(&self) -> anyhow::Result<()> {
        self.mpd.repeat()?;
        Ok(())
    }

    pub fn seek(&self, percentage: u8) -> anyhow::Result<()> {
        self.mpd.seek(percentage)?;
        self.mpd.pause(Some(true))?;
        self.mpd.pause(Some(false))?;
        Ok(())
    }

    pub fn status(&self) -> anyhow::Result<()> {
        let status = self.mpd.status()?;
        println!(
            "Pause: {}\tRandom: {}\tRepeat: {}",
            status.is_paused, status.random, status.repeat
        );
        Ok(())
    }

    pub fn queue(&self) -> anyhow::Result<()> {
        let queue = self.mpd.queue()?;
        for song in queue {
            let db_song = self.database.get_song_by_id(&song);
            if let Some(song_info) = db_song {
                println!(
                    "{} - {}",
                    song_info.name,
                    if let Some(artist) = song_info.artist {
                        artist
                    } else {
                        "Unknown".to_string()
                    }
                )
            }
        }
        Ok(())
    }

    pub fn add_to_queue(&self, song_name: &str) -> anyhow::Result<()> {
        let song_info = self.database.get_song_by_name(song_name);
        if let Some(song) = song_info {
            self.mpd.add_to_queue(&song.id)?;
            println!("Song {} added to queue", song_name);
        } else {
            return Err(anyhow!(format!("Song {} doesn't exist", song_name)));
        }
        Ok(())
    }

    pub fn remove_from_queue(&self, song_name: &str) -> anyhow::Result<()> {
        let song_info = self.database.get_song_by_name(song_name);
        if let Some(song) = song_info {
            self.mpd.remove_from_queue(&song.id)?;
            println!("Song {} removed from queue", song_name);
        } else {
            return Err(anyhow!(format!("Song {} doesn't exist", song_name)));
        }
        Ok(())
    }

    pub fn shuffle_queue(&self) -> anyhow::Result<()> {
        self.mpd.shuffle_queue()?;
        println!("Queue shuffled");
        Ok(())
    }
}
