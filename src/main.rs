mod args;
mod config;
mod db;
mod handler;
mod mpd;
mod utils;
mod youtube_api;

use clap::Parser;
use config::Config;
use mpd::mpd_client::MpdClient;

use crate::args::*;
use crate::db::database::Database;

use crate::handler::Handler;

use crate::youtube_api::youtube_api::YoutubeAPI;

fn main() {
    let config: Config = match confy::load("yap", "yap.config") {
        Ok(config) => config,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };
    let mpd = MpdClient::new();
    let database = match Database::new(format!("sqlite://{}", config.database.database_path), 10) {
        Ok(database) => database,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };
    database.run_embedded_migrations();
    let api = YoutubeAPI::new(config.general.music_directory.clone());
    let handler = Handler::new(database, api, mpd);
    let args = App::parse();

    match args.command {
        Command::Download(args) => parse_download_options(handler, args),
        Command::Playlist(args) => parse_playlist_options(handler, args),
        Command::Play(args) => parse_play_options(handler, args),
        Command::Song(args) => parse_song_options(handler, args),
        Command::Mpd(args) => parse_mpd_options(handler, args),
    }
}

fn parse_playlist_options(handler: Handler, options: PlaylistOptions) {
    match options {
        PlaylistOptions::Create(args) => {
            check!(handler.create_playlist(
                &args.name,
                args.songs
                    .split(",")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            ))
        }
        PlaylistOptions::List {} => {
            check!(handler.get_playlists());
        }
        PlaylistOptions::Delete(args) => check!(handler.delete_playlist(&args.name)),
        PlaylistOptions::Insert(args) => {
            check!(handler.insert_into_playlist(&args.playlist_name, &args.song_name))
        }
        PlaylistOptions::Remove(args) => {
            check!(handler.remove_song_from_playlist(&args.playlist_name, &args.song_name))
        }
    };
}
fn parse_download_options(handler: Handler, options: DownloadOptions) {
    check!(handler.add_song_to_registry(&options.song_id, &options.song_name, options.song_artist))
}

fn parse_play_options(handler: Handler, options: PlayOptions) {
    match options {
        PlayOptions::Playlist(args) => check!(handler.play_playlist(&args.name)),
        PlayOptions::Song(args) => check!(handler.play_song(&args.name)),
    }
}

fn parse_song_options(handler: Handler, options: SongOptions) {
    match options {
        SongOptions::List {} => check!(handler.get_songs()),
        SongOptions::Delete(args) => check!(handler.delete_song(&args.name)),
    }
}

pub fn parse_mpd_options(handler: Handler, options: MpdOptions) {
    match options {
        MpdOptions::Pause {} => check!(handler.pause()),
        MpdOptions::Shuffle {} => check!(handler.shuffle()),
        MpdOptions::Clear {} => check!(handler.clear_queue()),
        MpdOptions::Next {} => check!(handler.next()),
        MpdOptions::Current {} => check!(handler.current()),
        MpdOptions::Repeat {} => check!(handler.repeat()),
        MpdOptions::Previous {} => check!(handler.previous()),
    }
}
