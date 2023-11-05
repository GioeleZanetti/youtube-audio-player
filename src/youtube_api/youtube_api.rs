use std::{fs, path::Path};

use youtube_dl::YoutubeDl;

pub struct YoutubeAPI {
    music_directory: String,
}

impl YoutubeAPI {
    pub fn new(music_directory: String) -> Self {
        Self { music_directory }
    }

    pub fn download_audio(&self, path: &str) -> anyhow::Result<()> {
        YoutubeDl::new(path)
            .extract_audio(true)
            .output_template(path)
            .download_to(Path::new(&self.music_directory))?;
        Ok(())
    }

    pub fn delete_audio(&self, song_id: &str) -> anyhow::Result<()> {
        let path = &format!("{}/{}.opus", self.music_directory, song_id);
        fs::remove_file(Path::new(path))?;
        Ok(())
    }
}
