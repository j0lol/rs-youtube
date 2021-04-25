use crate::backend::youtube::sub_box::sub_box;
use crate::frontend::generic_menu::{
    enum_menu, AdditionalItem, MenuItems, ObjectItem, OrderedItem,
};
use crate::frontend::utils::clear_screen;
use crate::frontend::youtube::channel_view::show_channel;
use console::style;

pub fn show_sub_box() {
    clear_screen();
    println!("Fetching your subscriptions...");
    let vec = sub_box().unwrap();
    let mut new_vec = Vec::new();

    new_vec.push(ObjectItem {
        menu_item: MenuItems::TitleItem("Subscriptions:\n".to_string()),
        object: None,
    });

    for i in 0..vec.len() {
        new_vec.push(ObjectItem {
            menu_item: MenuItems::OrderedItem(OrderedItem {
                label: format!(
                    "{} {}\n{}\n",
                    style(&vec[i].channel_name).bold(),
                    style(&vec[i].video_timestamp).dim(),
                    vec[i].video_name
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
        Some(crate::backend::youtube::sub_box::FeedItem { channel_id, .. }) => {
            show_channel(channel_id.as_str())
        }
    };
}
