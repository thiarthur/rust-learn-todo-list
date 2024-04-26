mod menu;
mod models;
mod utils;

use menu::{create_menu, show_menu};
use models::Task;
use utils::clear_console;

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let menu = create_menu();

    let mut option_code: i8 = -1;

    while option_code != 0 {
        show_menu(&menu);
        let mut input = String::new();

        println!("\nChoose an option:");

        std::io::stdin().read_line(&mut input).unwrap();
        option_code = input.trim().parse().unwrap();

        println!("\n");
        clear_console();

        if let Some(option) = menu.iter().find(|&m| m.code == option_code) {
            (option.action)(&mut tasks);
        } else {
            println!("Option not found");
        }
    }
}
