use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub general: General,
    pub database: Database,
}
#[derive(Deserialize, Serialize)]
pub struct General {
    pub music_directory: String,
}

#[derive(Deserialize, Serialize)]
pub struct Database {
    pub database_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: General {
                music_directory: "~/Music/songs/".to_string(),
            },
            database: Database {
                database_path: "~/.config/yap/yap.db".to_string(),
            },
        }
    }
}
