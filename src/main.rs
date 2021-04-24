// Define modules & import them

// Load frontend/mod.rs
mod frontend;
// Load backend/mod.rs
mod backend;

fn main() {
    // Load main menu
    crate::frontend::main_menu::show_menu();
}
