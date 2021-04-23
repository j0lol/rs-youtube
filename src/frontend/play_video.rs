use crate::backend::utils::run_command;
use crate::frontend::play_video::VideoTypes::{Playlist, Video};

pub enum VideoTypes {
    Video(PlayerVideo),
    Playlist(PlayerList),
}
pub struct PlayerVideo {
    pub url: String,
}
pub struct PlayerList {
    pub url: String,
}

pub fn play_youtube_video(video: VideoTypes) {
    let url = match video {
        Video(PlayerVideo { url }) => url,
        Playlist(PlayerList { url }) => url,
    };
    println!("Playing {} :)", url);
    run_command(&format!("mpv {}", url));
}
