use crate::frontend::{twitch, youtube};
use crate::{frontend};

pub fn show_menu() {
    loop {
        match frontend::generic_menu::string_menu(vec![
            "Load YouTube Subscriptions",
            "Search YouTube",
            "Load Twitch Follows",
            "Search Twitch",
        ])
        .as_str()
        {
            "Exit menu" => break,
            "Load YouTube Subscriptions" => youtube::sub_box::show_sub_box(),
            "Search YouTube" => youtube::search::show_search(),
            "Load Twitch Follows" => twitch::follow_box::show_follow_box(),
            "Search Twitch" => twitch::search_channels::show_search(),
            string => println!("Please input a valid option. {}", string),
        }
    }
}
