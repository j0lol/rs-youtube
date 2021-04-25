use console::style;

use crate::backend::twitch::follow_box::{follow_box, ChannelStatus};
use crate::backend::twitch::search_channels::Channel;
use crate::frontend::generic_menu::{
    enum_menu, AdditionalItem, MenuItems, ObjectItem, OrderedItem,
};
use crate::frontend::twitch::search_channels::play_or_follow;
use crate::frontend::utils::clear_screen;

pub fn show_follow_box() {
    clear_screen();
    println!("Fetching your follows...");
    let vec = follow_box().unwrap();
    let mut new_vec = vec![ObjectItem {
        menu_item: MenuItems::TitleItem("Follows:\n".to_string()),
        object: None,
    }];

    for i in vec {
        new_vec.push(ObjectItem {
            menu_item: MenuItems::OrderedItem(OrderedItem {
                label: match &i.channel_status {
                    ChannelStatus::Live => {
                        format!(
                            "{} is playing {}\n{}\n",
                            style(&i.channel_display_name).green(),
                            &i.livestream_game.as_ref().unwrap(),
                            style(&i.livestream_title.as_ref().unwrap()).bold(),
                        )
                    }
                    ChannelStatus::Hosting => {
                        format!(
                            "{} is hosting {}\n{}\n",
                            style(&i.channel_display_name).yellow(),
                            &i.hosting_channel_display_name.as_ref().unwrap(),
                            style(&i.hosting_livestream_title.as_ref().unwrap()).bold(),
                        )
                    }
                    ChannelStatus::Offline => {
                        format!("{} is offline\n", style(&i.channel_display_name).red())
                    }
                },
                return_string: None,
            }),
            object: Some(i.clone()),
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
        None => {}
        Some(crate::backend::twitch::follow_box::FeedItem {
            channel_status,
            channel_name,
            channel_display_name,
            hosting_channel_name,
            hosting_channel_display_name,
            ..
        }) => match channel_status {
            ChannelStatus::Live | ChannelStatus::Offline => play_or_follow(
                Channel {
                    name: channel_name,
                    display_name: channel_display_name,
                },
                None,
            ),
            ChannelStatus::Hosting => play_or_follow(
                Channel {
                    name: channel_name,
                    display_name: channel_display_name,
                },
                Some(Channel {
                    name: hosting_channel_name.unwrap(),
                    display_name: hosting_channel_display_name.unwrap(),
                }),
            ),
        },
    };
}
