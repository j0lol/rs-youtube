use crate::backend::config::load_config;
use crate::backend::utils::run_command;

pub fn play_stream(channel_name: String) {
    let url = format!("https://twitch.tv/{}", channel_name);
    let player = load_config().unwrap();

    let chat_url= format!("{}{}", url, "/chat");

    match webbrowser::open(&chat_url) {
        Err(_) => {println!("Failed to open chat in browser...")}
        _ => {println!("Opening chat in browser...")}
    }

    let player = &(*player.options.video_player.as_str().unwrap());
    println!("▶ Now playing: {} :)", url);
    run_command(&format!("{} {}", player, url));
}
