use crate::backend::youtube::search::{perform_search, Results, Summary};
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

    pick_results(vec, format!("Search results for \"{}\":\n", input).as_str());
}

fn pick_results(vec: Vec<Results>, search_term: &str) {
    let mut new_vec = vec![ObjectItem {
        menu_item: MenuItems::TitleItem(search_term.to_string()),
        object: Results::None,
    }];
    for i in vec {
        // Push ObjectItem into vec
        new_vec.push(ObjectItem {
            // Give menu renderer information to render the video with:
            menu_item: (MenuItems::OrderedItem(OrderedItem {
                label: match &i {
                    Results::Video(video) => video.summarize(),
                    Results::Channel(channel) => channel.summarize(),
                    Results::Playlist(playlist) => playlist.summarize(),
                    Results::Shelf(shelf) => shelf.summarize(),
                    Results::None => "".to_string(),
                },
                return_string: None,
            })),
            object: (i.clone()),
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
        Results::Video(crate::backend::youtube::search::YoutubeVideo { id, .. }) => {
            crate::frontend::youtube::play_video::play_youtube_video(
                crate::frontend::youtube::play_video::VideoTypes::Video(
                    crate::frontend::youtube::play_video::PlayerVideo {
                        url: format!("https://youtube.com/watch?v={}", id),
                    },
                ),
            )
        }
        Results::Channel(crate::backend::youtube::search::YoutubeChannel { id, .. }) => {
            crate::frontend::youtube::channel_view::show_channel(id.as_str())
        }
        Results::Playlist(crate::backend::youtube::search::YoutubePlaylist { id, .. }) => {
            crate::frontend::youtube::play_video::play_youtube_video(
                crate::frontend::youtube::play_video::VideoTypes::Playlist(
                    crate::frontend::youtube::play_video::PlayerList {
                        url: format!("https://youtube.com/playlist?list={}", id),
                    },
                ),
            )
        }
        Results::Shelf(crate::backend::youtube::search::YoutubeShelf {
            title, content, ..
        }) => {
            println!("{}", title);
            pick_results(content, format!("{}\n", title).as_str())
        }
        Results::None => {}
    }
}
