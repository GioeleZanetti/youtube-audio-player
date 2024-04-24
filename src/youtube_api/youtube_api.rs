use std::{
    fs::{self, File},
    path::Path,
};

use youtube_dl::YoutubeDl;

pub struct YoutubeAPI {
    music_directory: String,
    miniature_directory: String,
    download_miniature: bool,
}

impl YoutubeAPI {
    pub fn new(
        music_directory: String,
        miniature_directory: String,
        download_miniature: bool,
    ) -> Self {
        Self {
            music_directory,
            miniature_directory,
            download_miniature,
        }
    }

    pub async fn download_audio(&self, path: &str, song_name: &str) -> anyhow::Result<()> {
        YoutubeDl::new(path)
            .extract_audio(true)
            .output_template(path)
            .download_to(Path::new(&self.music_directory))?;
        if self.download_miniature {
            let result = reqwest::get(format!("https://img.youtube.com/vi/{}/sddefault.jpg", path))
                .await
                .unwrap();
            let mut file = File::create(
                Path::new(&self.miniature_directory).join(format!("{}.jpg", song_name)),
            )
            .unwrap();
            std::io::copy(&mut result.bytes().await.unwrap().as_ref(), &mut file).unwrap();
        }
        Ok(())
    }

    pub fn delete_audio(&self, song_id: &str) -> anyhow::Result<()> {
        let path = format!("{}{}.opus", self.music_directory, song_id);
        fs::remove_file(Path::new(&path))?;
        Ok(())
    }
}
