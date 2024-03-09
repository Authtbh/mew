use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    categories: HashMap<String, Category>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Category {
    color: String,
    tasks: HashMap<usize, String>,
}

impl Todo {
    fn new() -> Self {
        Todo {
            categories: HashMap::new(),
        }
    }

    fn add_category(&mut self, category_name: String, color: String) {
        self.categories.insert(category_name.clone(), Category::new(color));
        self.save();
    }

    fn select_category(&mut self, category_name: &str) {
        if let Some(category) = self.categories.get(category_name) {
            let mut selected_category = Category::new(category.color.clone());
            selected_category.tasks.extend(category.tasks.clone());
            self.categories.clear();
            self.categories.insert(category_name.to_string(), selected_category);
            self.save();
        } else {
            println!("Category not found.");
        }
    }

    fn delete_category(&mut self, category_name: &str) {
        self.categories.remove(category_name);
        self.save();
        println!("Category '{}' deleted successfully.", category_name);
    }

    fn clear_all(&mut self) {
        self.categories.clear();
        self.save();
        println!("All categories and tasks deleted successfully.");
    }

    fn add_task(&mut self, task: String) {
        if let Some(category) = self.categories.values_mut().next() {
            let id = category.tasks.len() + 1;
            category.tasks.insert(id, task);
            self.save();
        } else {
            println!("Please select a category first using 'mew select <category>'.");
        }
    }

    fn list(&self) {
        for (category, tasks) in &self.categories {
            println!("Category: {} (Color: {})", category, tasks.color);
            for (id, task) in &tasks.tasks {
                println!("  {}: {}", id, task);
            }
        }
    }

    fn done(&mut self, id: usize) {
        if let Some(category) = self.categories.values_mut().next() {
            category.tasks.remove(&id);
            self.save();
        } else {
            println!("Please select a category first using 'mew select <category>'.");
        }
    }

    fn delete_all(&mut self) {
        if let Some(category) = self.categories.values_mut().next() {
            category.tasks.clear();
            self.save();
        } else {
            println!("Please select a category first using 'mew select <category>'.");
        }
    }

    fn save(&self) {
        let data = serde_json::to_string(self).unwrap();
        fs::write("todo.json", data).expect("Unable to write to file");
    }

    fn load() -> Self {
        if let Ok(data) = fs::read_to_string("todo.json") {
            if let Ok(todo) = serde_json::from_str(&data) {
                return todo;
            }
        }
        Todo::new()
    }
}

impl Category {
    fn new(color: String) -> Self {
        Category {
            color,
            tasks: HashMap::new(),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut todo = Todo::load();

    match args.get(1).map(String::as_str) {
        Some("new") => {
            if let (Some(category_name), Some(color)) = (args.get(2), args.get(3)) {
                todo.add_category(category_name.clone(), color.clone());
                println!("Category '{}' added successfully with color '{}'.", category_name, color);
            } else {
                println!("Usage: mew new <category_name> <color>");
            }
        }
        Some("select") => {
            if let Some(category_name) = args.get(2) {
                todo.select_category(category_name);
                println!("Selected category '{}'.", category_name);
            } else {
                println!("Usage: mew select <category_name>");
            }
        }
        Some("delete") => {
            if let Some(category_name) = args.get(2) {
                todo.delete_category(category_name);
            } else {
                println!("Usage: mew delete <category_name>");
            }
        }
        Some("clear") => {
            todo.clear_all();
        }
        Some("add") => {
            if let Some(task) = args.get(2) {
                todo.add_task(task.clone());
                println!("Task added successfully.");
            } else {
                println!("Usage: mew add <task>");
            }
        }
        Some("list") => {
            todo.list();
        }
        Some("done") => {
            if let Some(id_str) = args.get(2) {
                if let Ok(id) = id_str.parse::<usize>() {
                    todo.done(id);
                    println!("Task marked as done.");
                } else {
                    println!("Invalid task ID.");
                }
            } else {
                println!("Usage: mew done <task_id>");
            }
        }
        Some("delete_all") => {
            todo.delete_all();
            println!("All tasks deleted successfully.");
        }
        Some("-v") | Some("--version") => {
            println!("Mew CLI Helper v1.0 by Auth");
        }
        Some("-h") | Some("--help") => {
            println!("Usage: mew <new|select|delete|clear|add|list|done|delete_all> [arguments]");
            println!("Options:");
            println!("  -v, --version   Display version information");
            println!("  -h, --help      Display this help message");
            println!("\nCommands:");
            println!("  mew new <category_name> <color>        Create a new category");
            println!("  mew select <category_name>             Select a category");
            println!("  mew delete <category_name>             Delete a category");
            println!("  mew clear                              Clear all categories and tasks");
            println!("  mew add <task>                         Add a task to the selected category");
            println!("  mew list                               List tasks in the selected category");
            println!("  mew done <task_id>                     Mark a task as done");
            println!("  mew delete_all                         Delete all tasks in the selected category");
        }
        Some(_) => {
            println!("Invalid command. Use 'mew --help' for usage information.");
        }
        None => {
            println!("Welcome to Mew - Rust CLI Todo List");
            println!("Use 'mew --help' for usage information.");
        }
    }
}
