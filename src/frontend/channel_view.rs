use crate::backend::channel_view;
use crate::backend::channel_view::{ChannelResults, Summary, YoutubePlaylist, YoutubeVideo};
use crate::frontend::generic_menu::{
    enum_menu, AdditionalItem, MenuItems, ObjectItem, OrderedItem,
};
use crate::frontend::play_video::{play_youtube_video, PlayerList, PlayerVideo, VideoTypes};

pub fn show_channel(channel_id: &str) {
    let vec = crate::backend::channel_view::show_channel(channel_id);

    let mut new_vec = Vec::new();
    for i in 0..vec.len() {
        // Push ObjectItem into vec
        new_vec.push(ObjectItem {
            // Give menu renderer information to render the video with:
            menu_item: (MenuItems::OrderedItem(OrderedItem {
                label: match &vec[i] {
                    ChannelResults::Video(video) => video.summarize(),
                    ChannelResults::Playlist(playlist) => playlist.summarize(),
                    ChannelResults::None => "None".to_string(),
                },
                return_string: None,
            })),
            object: (vec[i].clone()),
        });
    }
    new_vec.push(ObjectItem {
        menu_item: MenuItems::AdditionalItem(AdditionalItem {
            input_label: String::from("exit"),
            label: String::from("Exit menu"),
        }),
        object: ChannelResults::None,
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
        None => {}
        Some(ChannelResults::None) => {}
    }
}
