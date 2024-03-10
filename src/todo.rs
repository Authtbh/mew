// todo.rs
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions}; // Import fs module and OpenOptions
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(description: String) -> Task {
        Task {
            description,
            completed: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    pub fn add(&mut self, task_description: String) {
        let task = Task::new(task_description);
        self.tasks.push(task);
        self.save().expect("Failed to save task.");
    }

    pub fn done(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.completed = true;
            self.save().expect("Failed to mark task as done.");
            println!("Task {} is marked as done.", index);
        } else {
            println!("Invalid task index.");
        }
    }

    pub fn list(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            println!(
                "{}: {} [{}]",
                index,
                task.description,
                if task.completed { "Done" } else { "Pending" }
            );
        }
    }

    pub fn delete(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
            self.save().expect("Failed to delete task.");
            println!("Task {} has been deleted.", index);
        } else {
            println!("Invalid task index.");
        }
    }

    fn save(&self) -> io::Result<()> {
        let data = serde_json::to_string(&self)?;
        fs::write("todo_list.json", data)?; // Use fs::write from the fs module
        Ok(())
    }

    pub fn load() -> TodoList {
        let path = "todo_list.json";
        if Path::new(path).exists() {
            let mut file = match OpenOptions::new().read(true).open(path) {
                Ok(file) => file,
                Err(e) => panic!("Problem opening the file: {:?}", e),
            };

            let mut data = String::new();
            match file.read_to_string(&mut data) {
                Ok(_) => serde_json::from_str(&data).unwrap_or_else(|_| TodoList::new()),
                Err(_) => TodoList::new(),
            }
        } else {
            TodoList::new()
        }
    }
}
