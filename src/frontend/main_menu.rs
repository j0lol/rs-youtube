use crate::{backend, frontend};

pub fn show_menu() {
    loop {
        match crate::frontend::generic_menu::string_menu(vec![
            "Load YouTube Subscriptions",
            "Search YouTube",
            "Load Twitch Follows",
            "Search Twitch",
        ])
        .as_str()
        {
            "Exit menu" => break,
            "Load YouTube Subscriptions" => frontend::sub_box::show_sub_box(),
            "Search YouTube" => frontend::search::show_search(),
            "Load Twitch Follows" => frontend::twitch::follow_box::show_follow_box(),
            "Search Twitch" => frontend::twitch::search_channels::show_search(),
            string => println!("Please input a valid option. {}", string),
        }
    }
}
