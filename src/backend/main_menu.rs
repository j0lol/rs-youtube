pub fn pick_menu_item(item: &str) {
    match item {
        "exit" => {}
        "Load Subscriptions" => crate::frontend::main_menu::show_menu(),
        "Search YouTube" => {}
        _ => {}
    }
}
