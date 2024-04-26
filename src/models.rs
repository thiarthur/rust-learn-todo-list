pub struct Task {
    pub title: String,
    pub description: String,
}

pub struct MenuOption {
    pub label: String,
    pub code: i8,
    pub action: fn(tasks: &mut Vec<Task>),
}
