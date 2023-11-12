use anyhow::anyhow;
use mpd::{Client, Song, State};

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

    pub fn repeat(&self) -> anyhow::Result<()> {
        let mut conn = Client::connect("127.0.0.1:6600")?;
        let repeat = !conn.status()?.repeat;
        conn.repeat(repeat)?;
        println!("Repeat: {}", repeat);
        Ok(())
    }
}
