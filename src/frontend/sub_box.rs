use crate::backend::sub_box::{sub_box, FeedItem};
use crate::frontend::channel_view::show_channel;
use crate::frontend::generic_menu::{enum_menu, MenuItems, OrderedItem};

pub fn show_sub_box() {
    println!("Loading...");
    let vec = sub_box().unwrap();
    let mut new_vec = Vec::new();
    for i in 0..vec.len() {
        new_vec.push(MenuItems::OrderedItem(OrderedItem {
            label: format!(
                "{} {}\n{}\n",
                vec[i].channel_name, vec[i].video_timestamp, vec[i].video_name
            ),
            return_string: Some(format!("{}", &vec[i].channel_id)),
        }));
    }
    let id = enum_menu(new_vec);
    show_channel(id.as_str());
}
