use std::io;

use crate::backend::config::{follow, is_following, unfollow};
use crate::backend::twitch::search_channels::{perform_search, Channel};
use crate::frontend::generic_menu::{
    enum_menu, AdditionalItem, MenuItems, ObjectItem, OrderedItem,
};
use crate::frontend::twitch::play_stream::play_stream;

pub fn show_search() {
    crate::frontend::utils::clear_screen();
    println!("Search twitch for channels:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    let vec = perform_search(input.to_string()).unwrap();

    pick_results(vec, format!("Search results for \"{}\":\n", input).as_str());
}

fn pick_results(vec: Vec<Channel>, search_term: &str) {
    let mut new_vec = Vec::new();
    new_vec.push(ObjectItem {
        menu_item: MenuItems::TitleItem(search_term.to_string()),
        object: None,
    });
    for i in 0..vec.len() {
        // Push ObjectItem into vec
        new_vec.push(ObjectItem {
            // Give menu renderer information to render the video with:
            menu_item: (MenuItems::OrderedItem(OrderedItem {
                label: format!("{}", &vec[i].display_name),
                return_string: None,
            })),
            object: Some(vec[i].clone()),
        });
    }
    new_vec.push(ObjectItem {
        menu_item: MenuItems::AdditionalItem(AdditionalItem {
            input_label: String::from("exit"),
            label: String::from("Exit menu"),
        }),
        object: None,
    });
    match enum_menu(new_vec).unwrap() {
        Some(channel) => play_or_follow(channel, None),
        None => {}
    }
}

pub fn play_or_follow(channel: Channel, go_to_channel: Option<Channel>) {
    let formatted_menu_item = match &go_to_channel {
        None => {
            format!("Watch {}", channel.display_name)
        }
        Some(label_channel) => {
            format!("Watch {}", label_channel.display_name)
        }
    };

    let x =
        crate::frontend::generic_menu::string_menu(vec![formatted_menu_item.as_str(), "Un/follow"]);

    if x == formatted_menu_item {
        match &go_to_channel {
            None => play_stream(channel.name),
            Some(label_channel) => play_stream((&label_channel.name).to_string()),
        }
    } else if x == "Un/follow" {
        if is_following((&channel.name).to_string()) {
            unfollow(channel.name.clone());
        } else {
            follow(channel.name.clone());
        }
    } else {
        println!("Please input a valid option. {}", x);
    }
}
