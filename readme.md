# Mew - Rust CLI Todo List

Mew is a command-line interface (CLI) Todo List application written in Rust. It offers a minimalistic yet effective way to manage your tasks and stay organized.







##Installation

- **For Windows**
- Download the release mew.exe and paste it into `C:\Windows\System32`


- **For Linux**
- Download the release mew.exe and paste it into `/usr/local/bin`



-**Soon for Mac**









## Features

- **Category Management:**
  - Create, select, and delete customizable categories.
  - Assign a unique color to each category for easy identification.

- **Task Management:**
  - Add, list, mark tasks as done, and delete tasks within selected categories.
  - Tasks are uniquely identified by numerical IDs.

- **User-Friendly Commands:**
  - Straightforward commands for creating categories, adding tasks, and more.

- **Persistent Storage:**
  - Tasks and categories are stored persistently in a JSON file.

- **Version Information and Help:**
  - Retrieve version information (`mew -v` or `mew --version`).
  - Get help with available commands and options (`mew -h` or `mew --help`).

## Getting Started

1. Clone the repository.
2. Build the project using `cargo build`.
3. Run Mew using `cargo run`.

For detailed usage instructions, explore the help message with `mew --help`.

## Example Commands

- Create a new category:
  ```bash
mew add "Finish report" 
```

##For more type mew -help


##Version history

- v1 initial commit