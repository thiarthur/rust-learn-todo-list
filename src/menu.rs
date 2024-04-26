use crate::models::{MenuOption, Task};
use std::process;

pub fn show_menu(menu: &Vec<MenuOption>) {
    for item in menu.iter() {
        println!("{}. {}", item.code, item.label);
    }
}

pub fn create_menu() -> Vec<MenuOption> {
    let mut menu: Vec<MenuOption> = Vec::new();

    menu.push(MenuOption {
        label: "Create".to_string(),
        code: 1,
        action: create_task,
    });

    menu.push(MenuOption {
        label: "List".to_string(),
        code: 2,
        action: list_tasks,
    });

    menu.push(MenuOption {
        label: "Remove".to_string(),
        code: 3,
        action: remove_task,
    });

    menu.push(MenuOption {
        label: "Edit".to_string(),
        code: 4,
        action: edit_task,
    });

    menu.push(MenuOption {
        label: "Exit".to_string(),
        code: 9,
        action: exit,
    });

    return menu;
}

pub fn create_task(tasks: &mut Vec<Task>) {
    println!("Creating task...");
    let mut title = String::new();
    let mut description = String::new();

    println!("Enter the title of the task:");
    std::io::stdin().read_line(&mut title).unwrap();

    println!("Enter the description of the task:");
    std::io::stdin().read_line(&mut description).unwrap();

    let task = Task {
        title: title.trim().to_string(),
        description: description.trim().to_string(),
    };

    tasks.push(task);
    println!("Task created successfully\n\n");
}

pub fn remove_task(tasks: &mut Vec<Task>) {
    list_tasks(tasks);
    let mut index = String::new();

    println!("Enter the index of the task to remove:");
    std::io::stdin().read_line(&mut index).unwrap();

    let index: usize = index.trim().parse().unwrap();

    if index < tasks.len() {
        tasks.remove(index);
        println!("Task removed successfully\n\n");
    } else {
        println!("Task not found\n\n");
    }
}

pub fn list_tasks(tasks: &mut Vec<Task>) {
    if tasks.len() == 0 {
        println!("No tasks found\n\n");
        return;
    }

    for (index, task) in tasks.iter().enumerate() {
        println!("Task #{}", index);
        println!("Title: {}", task.title);
        println!("Description: {}", task.description);
        println!("\n");
    }
}

pub fn edit_task(tasks: &mut Vec<Task>) {
    list_tasks(tasks);
    let mut index = String::new();

    println!("Enter the index of the task to edit:");
    std::io::stdin().read_line(&mut index).unwrap();

    let index: usize = index.trim().parse().unwrap();

    if index < tasks.len() {
        let mut title = String::new();
        let mut description = String::new();

        println!("Enter the new title of the task:");
        std::io::stdin().read_line(&mut title).unwrap();

        println!("Enter the new description of the task:");
        std::io::stdin().read_line(&mut description).unwrap();

        tasks[index].title = title.trim().to_string();
        tasks[index].description = description.trim().to_string();

        println!("Task edited successfully\n\n");
    } else {
        println!("Task not found\n\n");
    }
}

pub fn exit(tasks: &mut Vec<Task>) {
    println!("All {} tasks will be deleted", tasks.len());
    println!("Exiting...");
    process::exit(0); // Exit with a success code
}
