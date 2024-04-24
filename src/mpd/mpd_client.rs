use std::time::Duration;

use anyhow::anyhow;
use mpd::{Client, Song, State};

pub struct Status {
    pub repeat: bool,
    pub random: bool,
    pub is_paused: bool,
}

pub struct MpdClient {}

impl MpdClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update_db(&self) -> anyhow::Result<()> {
        let mut conn = Client::connect("127.0.0.1:6600")?;
        conn.update()?;
        Ok(())
    }

    pub fn add_to_queue(&self, song_id: &str) -> anyhow::Result<()> {
        let mut conn = Client::connect("127.0.0.1:6600")?;
        let song = Song {
            file: format!("{}.opus", song_id).to_string(),
            ..Default::default()
        };
        conn.push(song)?;
        Ok(())
    }

    pub fn play(&self) -> anyhow::Result<()> {
        let mut conn = Client::connect("127.0.0.1:6600")?;
        conn.play()?;
        Ok(())
    }

    pub fn pause(&self, state: Option<bool>) -> anyhow::Result<()> {
        let mut conn = Client::connect("127.0.0.1:6600")?;
        let new_state = match conn.status()?.state {
            mpd::State::Stop => false,
            mpd::State::Play => true,
            mpd::State::Pause => false,
        };
        if let Some(state) = state {
            conn.pause(state)?;
        } else {
            conn.pause(new_state)?;
            println!("Pause: {}", conn.status()?.state == State::Pause);
        }

        Ok(())
    }

    pub fn shuffle(&self, state: Option<bool>) -> anyhow::Result<()> {
        let mut conn = Client::connect("127.0.0.1:6600")?;
        let new_state = !conn.status()?.random;
        if let Some(state) = state {
            conn.random(state)?;
        } else {
            conn.random(new_state)?;
        }
        println!("Shuffle: {}", conn.status()?.random);
        Ok(())
    }

    pub fn clear_queue(&self) -> anyhow::Result<()> {
        let mut conn = Client::connect("127.0.0.1:6600")?;
        conn.clear()?;
        Ok(())
    }

    pub fn next(&self) -> anyhow::Result<()> {
        let mut conn = Client::connect("127.0.0.1:6600")?;
        conn.next()?;
        Ok(())
    }

    pub fn previous(&self) -> anyhow::Result<()> {
        let mut conn = Client::connect("127.0.0.1:6600")?;
        conn.prev()?;
        Ok(())
    }

    pub fn current(&self) -> anyhow::Result<String> {
        let mut conn = Client::connect("127.0.0.1:6600")?;
        if let Some(song) = conn.currentsong()? {
            Ok(song.file)
        } else {
            Err(anyhow!("No song currently playing"))
        }
    }

    pub fn current_time(&self) -> anyhow::Result<String> {
        let mut conn = Client::connect("127.0.0.1:6600")?;
        if let Some(timestamps) = conn.status()?.time {
            let perc = timestamps.0.as_secs() as f64 / timestamps.1.as_secs() as f64 * 100.;
            let current_minute = timestamps.0.as_secs() / 60;
            let current_second = timestamps.0.as_secs() - current_minute * 60;
            let song_minutes = timestamps.1.as_secs() / 60;
            let song_seconds = timestamps.1.as_secs() - song_minutes * 60;
            Ok(format!(
                "{}:{}/{}:{} ({}%)",
                current_minute, current_second, song_minutes, song_seconds, perc as i32
            ))
        } else {
            Err(anyhow!("No song currently playing"))
        }
    }

    pub fn repeat(&self) -> anyhow::Result<()> {
        let mut conn = Client::connect("127.0.0.1:6600")?;
        let repeat = !conn.status()?.repeat;
        conn.repeat(repeat)?;
        println!("Repeat: {}", repeat);
        Ok(())
    }

    pub fn seek(&self, perc: u8) -> anyhow::Result<()> {
        let mut conn = Client::connect("127.0.0.1:6600")?;
        if let Some(current_song) = conn.currentsong()? {
            let current_song_in_queue = conn
                .queue()?
                .into_iter()
                .filter(|song| song.file == current_song.file)
                .collect::<Vec<Song>>();
            if current_song_in_queue.len() != 1 {
                return Err(anyhow!("No song currently playling"));
            }
            let current_song_in_queue = &current_song_in_queue[0];
            let queue_place = current_song_in_queue.place.unwrap();
            let seconds_to_skip_to = conn.status()?.time.unwrap().1.as_secs() * perc as u64 / 100;
            conn.seek(queue_place.id, Duration::from_secs(seconds_to_skip_to))?;

            println!("Seeked to {}%", perc);
        }
        Ok(())
    }

    pub fn status(&self) -> anyhow::Result<Status> {
        let mut conn = Client::connect("127.0.0.1:6600")?;
        let status = conn.status()?;
        let out = Status {
            random: status.random,
            repeat: status.repeat,
            is_paused: match status.state {
                State::Play => false,
                _ => true,
            },
        };
        Ok(out)
    }

    pub fn queue(&self) -> anyhow::Result<Vec<String>> {
        let mut conn = Client::connect("127.0.0.1:6600")?;
        let songs = conn.queue()?.into_iter().collect::<Vec<Song>>();
        let mut filenames = Vec::new();
        for song in songs {
            let index = song.file.chars().position(|c| c == '.').unwrap();
            filenames.push(song.file[..index].to_string());
        }
        Ok(filenames)
    }

    pub fn remove_from_queue(&self, song_id: &str) -> anyhow::Result<()> {
        let mut conn = Client::connect("127.0.0.1:6600")?;
        let song_in_queue_pos = conn
            .queue()?
            .into_iter()
            .filter(|song| song.file == format!("{}.opus", song_id))
            .collect::<Vec<Song>>();
        if song_in_queue_pos.len() > 0 {
            let position = song_in_queue_pos[0].place.unwrap().id;
            conn.delete(position)?;
        }
        Ok(())
    }

    pub fn shuffle_queue(&self) -> anyhow::Result<()> {
        let mut conn = Client::connect("127.0.0.1:6600")?;
        conn.shuffle(..)?;
        Ok(())
    }
}
