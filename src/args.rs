use clap::{Args, Parser, Subcommand};

///YAP cli
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct App {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    ///Download options
    Download(DownloadOptions),
    ///Playlist options
    #[clap(subcommand)]
    Playlist(PlaylistOptions),
    ///Play options
    #[clap(subcommand)]
    Play(PlayOptions),
    ///song options
    #[clap(subcommand)]
    Song(SongOptions),
    ///Mpd daemon options
    #[clap(subcommand)]
    Mpd(MpdOptions),
}

#[derive(Debug, Args)]
pub struct DownloadOptions {
    ///specify url of song to download
    #[arg(long, short = 'i', requires = "song_name")]
    pub song_id: String,

    ///specify name of song to download
    #[arg(long, short = 'n', requires = "song_id")]
    pub song_name: String,
}

#[derive(Debug, Subcommand)]
pub enum PlaylistOptions {
    ///Create playlist
    Create(PlaylistCreateOptions),
    ///List available playlists
    List {},
    ///Delete playlist
    Delete(PlaylistDeleteOptions),
    ///Insert song into playlist
    Insert(PlaylistInsertOptions),
    ///Delete song from playlist
    Remove(PlaylistRemoveOptions),
}

#[derive(Debug, Args)]
pub struct PlaylistRemoveOptions {
    ///The playlist name
    #[arg(long, short, requires = "song_name")]
    pub playlist_name: String,
    ///Songs to remove from the playlist
    #[arg(long, short, requires = "playlist_name")]
    pub song_name: String,
}

#[derive(Debug, Args)]
pub struct PlaylistCreateOptions {
    ///The playlist name
    #[arg(long, short, requires = "songs")]
    pub name: String,

    ///Songs to add to the playlist, separated by comma
    #[arg(long, short, requires = "name")]
    pub songs: String,
}

#[derive(Debug, Args)]
pub struct PlaylistInsertOptions {
    ///Name of the playlist to insert the song
    #[arg(long, short, requires = "song_name")]
    pub playlist_name: String,
    ///Name of the song to add to the playlist
    #[arg(long, short, requires = "playlist_name")]
    pub song_name: String,
}

#[derive(Debug, Args)]
pub struct PlaylistDeleteOptions {
    ///The playlist name
    #[arg(long, short)]
    pub name: String,
}

#[derive(Debug, Subcommand)]
pub enum PlayOptions {
    ///Select playlist to play
    Playlist(PlayPlaylistOptions),
    ///Select song to play
    Song(PlaySongOptions),
}

#[derive(Debug, Args)]
pub struct PlayPlaylistOptions {
    ///The playlist name
    #[arg(long, short)]
    pub name: String,
}

#[derive(Debug, Args)]
pub struct PlaySongOptions {
    ///The song name name
    #[arg(long, short)]
    pub name: String,
}

#[derive(Debug, Subcommand)]
pub enum SongOptions {
    ///List available songs
    List {},
    ///Delete song
    Delete(SongDeleteOptions),
}

#[derive(Debug, Args)]
pub struct SongDeleteOptions {
    ///Name of the song to delete
    #[arg(long, short)]
    pub name: String,
}

#[derive(Debug, Subcommand)]
pub enum MpdOptions {
    ///Toggles pause funtion
    Pause {},
    ///Toggles shuffle funtion
    Shuffle {},
    ///Clears current queue
    Clear {},
    ///Skips to next song in queue
    Next {},
}
