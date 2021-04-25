use crate::backend::config::load_config;
use crate::backend::utils::run_command;


pub fn play_stream(channel_name: String) {
    let url = format!("https://twitch.tv/{}", channel_name);
    let player = load_config().unwrap();
    let player = player.video_player.as_str().unwrap().clone();
    println!("▶ Now playing: {} :)", url);
    run_command(&format!("{} {}", player, url));
}
