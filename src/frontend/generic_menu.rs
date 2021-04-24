use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
pub enum MenuItems {
    OrderedItem(OrderedItem),
    AdditionalItem(AdditionalItem),
}

/// OrderedItem: An item that will be labelled and chosen with a number, in order from 0 onwards.
#[derive(Debug, Clone)]
pub struct OrderedItem {
    pub(crate) label: String,
    pub(crate) return_string: Option<String>,
}
/// AdditionalItem: An item that will be labelled and chosen with a string, placed wherever.
#[derive(Debug, Clone)]
pub struct AdditionalItem {
    pub(crate) input_label: String,
    pub(crate) label: String,
}
#[derive(Debug, Clone)]
pub struct ObjectItem<T>
where
    T: Clone,
{
    pub(crate) menu_item: MenuItems,
    pub(crate) object: T,
}

pub fn enum_menu<T: Clone>(vec: Vec<ObjectItem<T>>) -> Option<T> {
    crate::frontend::utils::clear_screen();
    let mut menu_items: HashMap<String, T> = HashMap::new();

    // "Render" menu and add items to hash map
    for i in 0..vec.len() {
        let object = vec[i].clone();
        println!(
            "{}",
            match object.menu_item {
                MenuItems::OrderedItem(item) => {
                    menu_items.insert(i.to_string(), object.object);
                    format!("{}) {}", i, item.label)
                }
                MenuItems::AdditionalItem(item) => {
                    menu_items.insert(item.input_label.clone(), object.object);
                    format!("{}) {}", item.input_label, item.label)
                }
            }
        );
    }
    println!("Select an item.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    return match menu_items.get(input) {
        Some(item) => Some(item.clone()),
        None => None,
    };
}

pub fn string_menu(vec: Vec<&str>) -> String {
    match enum_menu(string_vec_to_enum_vec(vec)) {
        Some(string) => string,
        None => String::from("Invalid input."),
    }
}

fn string_vec_to_enum_vec(vec: Vec<&str>) -> Vec<ObjectItem<String>> {
    let mut out_vec: Vec<ObjectItem<String>> = Vec::new();

    // Turn strings into menu items
    for i in 0..vec.len() {
        out_vec.push(ObjectItem {
            menu_item: MenuItems::OrderedItem(OrderedItem {
                label: vec[i].to_string(),
                return_string: None,
            }),
            object: vec[i].to_string(),
        })
    }
    // Add exit item
    out_vec.push(ObjectItem {
        menu_item: MenuItems::AdditionalItem(AdditionalItem {
            input_label: String::from("exit"),
            label: String::from("Exit menu"),
        }),
        object: "Exit menu".to_string(),
    });

    out_vec
}
