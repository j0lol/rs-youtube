use crate::frontend;

pub fn show_menu() {
    loop {
        match crate::frontend::generic_menu::string_menu(vec![
            "Load Subscriptions",
            "Search YouTube",
        ])
        .as_str()
        {
            "Exit menu" => break,
            "Load Subscriptions" => frontend::sub_box::show_sub_box(),
            "Search YouTube" => frontend::search::show_search(),
            string => println!("Please input a valid option. {}", string),
        }
    }
}
