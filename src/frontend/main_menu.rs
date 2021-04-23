use crate::frontend;
use crate::frontend::generic_menu::string_menu;
use std::io;

pub fn show_menu() {
    loop {
        match string_menu(vec!["Load Subscriptions", "Search YouTube"]).as_str() {
            "exit" => break,
            "Load Subscriptions" => frontend::sub_box::show_sub_box(),
            "Search YouTube" => frontend::search::show_search(),
            string => println!("Please input a valid option. {}", string),
        }
    }
}
