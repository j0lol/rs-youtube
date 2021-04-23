use crate::backend::channel_view;
use crate::backend::channel_view::{ChannelResults, Summary};
use crate::frontend::generic_menu::{enum_menu, MenuItems, OrderedItem};

pub fn show_channel(channel_id: &str) {
    let vec = crate::backend::channel_view::show_channel(channel_id);

    println!("test");

    let mut new_vec = Vec::new();
    for i in 0..vec.len() {
        new_vec.push(MenuItems::OrderedItem(OrderedItem {
            label: match &vec[i] {
                channel_view::ChannelResults::Video(video) => video.summarize(),
                channel_view::ChannelResults::Playlist(playlist) => playlist.summarize(),
                channel_view::ChannelResults::None => String::from("None"),
            },
            return_string: None,
        }));
    }
    println!("{}", enum_menu(new_vec));
}
