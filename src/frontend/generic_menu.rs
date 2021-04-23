use std::io;

pub enum MenuItems {
    OrderedItem(OrderedItem),
    AdditionalItem(AdditionalItem),
}
pub struct OrderedItem {
    pub(crate) label: String,
    pub(crate) return_string: Option<String>,
}
pub struct AdditionalItem {
    input_label: String,
    label: String,
}

fn string_vec_to_enum_vec(vec: Vec<&str>) -> Vec<MenuItems> {
    let mut out_vec: Vec<MenuItems> = Vec::new();
    for i in 0..vec.len() {
        out_vec.push(MenuItems::OrderedItem(OrderedItem {
            label: vec[i].parse().unwrap(),
            return_string: None,
        }))
    }
    out_vec.push(MenuItems::AdditionalItem(AdditionalItem {
        input_label: String::from("exit"),
        label: String::from("Exit menu"),
    }));
    out_vec
}

pub fn enum_menu(vec: Vec<MenuItems>) -> String {
    let mut additional_item_vec = Vec::new();
    for i in 0..vec.len() {
        match &vec[i] {
            MenuItems::OrderedItem(item) => {
                println!("{}) {}", i, item.label)
            }
            MenuItems::AdditionalItem(item) => {
                additional_item_vec.push(item.input_label.clone());
                println!("{}) {}", item.input_label, item.label)
            }
        }
    }
    println!("Select an item.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    if additional_item_vec.iter().any(|i| i == input) {
        return input.to_string();
    } else {
        let input: usize = input.parse().unwrap();
        if input < vec.len() {
            match &vec[input] {
                MenuItems::OrderedItem(item) => match item.return_string {
                    None => return item.label.to_string(),
                    Some(_) => return item.return_string.clone().unwrap(),
                },
                MenuItems::AdditionalItem(item) => &item.input_label,
            }
        } else {
            let string = String::from("not a channel");
            return string;
        }
    }
    .to_string()
    // Have input, check if item is valid via match and if
}
pub fn string_menu(vec: Vec<&str>) -> String {
    enum_menu(string_vec_to_enum_vec(vec))
}
