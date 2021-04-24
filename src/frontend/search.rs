use crate::backend::search::{perform_search, Results, Summary};
use crate::frontend::generic_menu::{
    enum_menu, AdditionalItem, MenuItems, ObjectItem, OrderedItem,
};
use std::io;

pub fn show_search() {
    crate::frontend::utils::clear_screen();
    println!("Search youtube:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    let vec = perform_search(input.to_string()).unwrap();

    pick_results(vec);
}

fn pick_results(vec: Vec<Results>) {
    let mut new_vec = Vec::new();
    for i in 0..vec.len() {
        // Push ObjectItem into vec
        new_vec.push(ObjectItem {
            // Give menu renderer information to render the video with:
            menu_item: (MenuItems::OrderedItem(OrderedItem {
                label: match &vec[i] {
                    Results::Video(video) => video.summarize(),
                    Results::Channel(channel) => channel.summarize(),
                    Results::Playlist(playlist) => playlist.summarize(),
                    Results::Shelf(shelf) => shelf.summarize(),
                    Results::None => "".to_string(),
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
        object: Results::None,
    });
    match enum_menu(new_vec).unwrap() {
        Results::Video(crate::backend::search::YoutubeVideo { id, .. }) => {
            crate::frontend::play_video::play_youtube_video(
                crate::frontend::play_video::VideoTypes::Video(
                    crate::frontend::play_video::PlayerVideo {
                        url: format!("https://youtube.com/watch?v={}", id),
                    },
                ),
            )
        }
        Results::Channel(crate::backend::search::YoutubeChannel { id, .. }) => {
            crate::frontend::channel_view::show_channel(id.as_str())
        }
        Results::Playlist(crate::backend::search::YoutubePlaylist { id, .. }) => {
            crate::frontend::play_video::play_youtube_video(
                crate::frontend::play_video::VideoTypes::Playlist(
                    crate::frontend::play_video::PlayerList {
                        url: format!("https://youtube.com/playlist?list={}", id),
                    },
                ),
            )
        }
        Results::Shelf(crate::backend::search::YoutubeShelf { title, content, .. }) => {
            println!("{}", title);
            pick_results(content)
        }
        Results::None => {}
    }
}
