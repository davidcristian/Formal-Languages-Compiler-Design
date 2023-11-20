mod models;
mod tests;
mod utils;
mod view;

use view::menu::Menu;

fn main() {
    // create a new menu and show it
    let main_menu = Menu::new();
    main_menu.show();

    // this will be printed after the user closes the menu
    println!("Done!");
}
