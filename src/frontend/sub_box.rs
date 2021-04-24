use crate::backend::sub_box::{sub_box};
use crate::frontend::channel_view::show_channel;
use crate::frontend::generic_menu::{
    enum_menu, AdditionalItem, MenuItems, ObjectItem, OrderedItem,
};

pub fn show_sub_box() {
    println!("Loading...");
    let vec = sub_box().unwrap();
    let mut new_vec = Vec::new();
    for i in 0..vec.len() {
        new_vec.push(ObjectItem {
            menu_item: MenuItems::OrderedItem(OrderedItem {
                label: format!(
                    "{} {}\n{}\n",
                    vec[i].channel_name, vec[i].video_timestamp, vec[i].video_name
                ),
                return_string: None,
            }),
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
        None => {}
        Some(crate::backend::sub_box::FeedItem { channel_id, .. }) => {
            show_channel(channel_id.as_str())
        }
    };
}
