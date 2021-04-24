use crate::backend::channel_view::{ChannelResults, Summary, YoutubePlaylist, YoutubeVideo};
use crate::backend::config::{is_subscribed, subscribe, unsubscribe};
use crate::frontend::generic_menu::{
    enum_menu, AdditionalItem, MenuItems, ObjectItem, OrderedItem,
};
use crate::frontend::play_video::{play_youtube_video, PlayerList, PlayerVideo, VideoTypes};
use console::style;

pub fn show_channel(channel_id: &str) {
    let vec = crate::backend::channel_view::show_channel(channel_id).2;

    let channel_name = crate::backend::channel_view::show_channel(channel_id).0;
    let channel_subs = crate::backend::channel_view::show_channel(channel_id).1;

    let subscribed = is_subscribed(channel_id.to_string());
    let subscribed = match subscribed {
        true => "Subscribed",
        false => "Not subscribed",
    };

    let mut new_vec: Vec<ObjectItem<ChannelResults>> = Vec::new();

    new_vec.push(ObjectItem {
        menu_item: MenuItems::TitleItem(format!(
            "{}\n{} {}\n",
            style(channel_name).bold(),
            channel_subs,
            style(subscribed).dim()
        )),
        object: ChannelResults::None("Title".to_string()),
    });

    for i in 0..vec.len() {
        // Push ObjectItem into vec
        new_vec.push(ObjectItem {
            // Give menu renderer information to render the video with:
            menu_item: (MenuItems::OrderedItem(OrderedItem {
                label: match &vec[i] {
                    ChannelResults::Video(video) => video.summarize(),
                    ChannelResults::Playlist(playlist) => playlist.summarize(),
                    ChannelResults::None(_) => "None".to_string(),
                },
                return_string: None,
            })),
            object: (vec[i].clone()),
        });
    }
    if is_subscribed(channel_id.to_string()) {
        new_vec.push(ObjectItem {
            menu_item: MenuItems::AdditionalItem(AdditionalItem {
                input_label: String::from("unsub"),
                label: String::from("Unsubscribe"),
            }),
            object: ChannelResults::None("Unsubscribe".to_string()),
        });
    } else {
        new_vec.push(ObjectItem {
            menu_item: MenuItems::AdditionalItem(AdditionalItem {
                input_label: String::from("sub"),
                label: String::from("Subscribe"),
            }),
            object: ChannelResults::None("Subscribe".to_string()),
        });
    }

    new_vec.push(ObjectItem {
        menu_item: MenuItems::AdditionalItem(AdditionalItem {
            input_label: String::from("exit"),
            label: String::from("Exit menu"),
        }),
        object: ChannelResults::None("Exit menu".to_string()),
    });

    match enum_menu(new_vec) {
        Some(ChannelResults::Video(YoutubeVideo { id, .. })) => {
            play_youtube_video(VideoTypes::Video(PlayerVideo {
                url: format!("https://youtube.com/watch?v={}", id),
            }))
        }
        Some(ChannelResults::Playlist(YoutubePlaylist { url, .. })) => {
            play_youtube_video(VideoTypes::Playlist(PlayerList {
                url: format!("https://youtube.com{}", url),
            }))
        }
        Some(ChannelResults::None(string)) => match string.as_str() {
            "Exit menu" => {}
            "Subscribe" => subscribe(String::from(channel_id)),
            "Unsubscribe" => unsubscribe(String::from(channel_id)),
            _ => {}
        },
        _ => {}
    }
}
