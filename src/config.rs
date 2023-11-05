use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub general: General,
    pub database: Database,
}
#[derive(Deserialize)]
pub struct General {
    pub music_directory: String,
}

#[derive(Deserialize)]
pub struct Database {
    pub database_path: String,
}
