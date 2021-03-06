use crate::backend::config::load_config;
use crate::backend::utils::run_command;
use crate::frontend::youtube::play_video::VideoTypes::{Playlist, Video};

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
    crate::frontend::utils::clear_screen();
    let url = match video {
        Video(PlayerVideo { url }) => url,
        Playlist(PlayerList { url }) => url,
    };
    let player = load_config().unwrap();
    let player = &(*player.options.video_player.as_str().unwrap());
    println!("▶ Now playing: {} :)", url);
    run_command(&format!("{} {}", player, url));
}
