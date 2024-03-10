// cli.rs
use crate::todo::TodoList;
use std::env;

pub fn handle_cli_commands() {
    let args: Vec<String> = env::args().collect();
    let mut todo_list = TodoList::load();

    match args.get(1).map(String::as_str) {
        Some("add") if args.len() > 2 => {
            let description = args[2..].join(" ");
            todo_list.add(description);
        }
        Some("list") => {
            todo_list.list();
        }
        Some("done") if args.len() > 2 => {
            if let Ok(index) = args[2].parse::<usize>() {
                todo_list.done(index);
            } else {
                println!("Please provide a valid task index.");
            }
        }
        Some("delete") if args.len() > 2 => {
            if let Ok(index) = args[2].parse::<usize>() {
                todo_list.delete(index);
            } else {
                println!("Please provide a valid task index.");
            }
        }
        Some("-h") => {
            print_help();
        }
        Some("-v") => {
            print_version();
        }
        _ => {
            println!("Usage: mew <add|list|done|delete> [arguments]");
            println!("       mew -h : Display help");
            println!("       mew -v : Display version");
        }
    }
}

fn print_help() {
    println!("Usage: mew <add|list|done|delete> [arguments]");
    println!("       mew -h : Display help");
    println!("       mew -v : Display version");
}

fn print_version() {
    println!("mew v1.0.0");
}
