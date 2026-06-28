use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

fn data_path() -> PathBuf {
    let mut path = env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));
    path.push(".todo.json");
    path
}

fn load_tasks() -> Vec<Task> {
    let path = data_path();
    fs::read_to_string(&path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default()
}

fn save_tasks(tasks: &[Task]) {
    let data = serde_json::to_string_pretty(tasks).unwrap();
    fs::write(data_path(), data).expect("failed to save tasks");
}

fn next_id(tasks: &[Task]) -> u32 {
    tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

fn print_usage() {
    println!("Usage: todo <command> [arguments]");
    println!();
    println!("Commands:");
    println!("  add <text>       — add a task");
    println!("  list              — show all tasks");
    println!("  done <id>         — mark task as done");
    println!("  remove <id>       — remove a task");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        print_usage();
        return;
    }

    match args[0].as_str() {
        "add" => {
            let title = args[1..].join(" ");
            if title.is_empty() {
                eprintln!("specify task text");
                return;
            }
            let mut tasks = load_tasks();
            let task = Task {
                id: next_id(&tasks),
                title,
                done: false,
            };
            println!("task #{} added: {}", task.id, task.title);
            tasks.push(task);
            save_tasks(&tasks);
        }
        "list" => {
            let tasks = load_tasks();
            if tasks.is_empty() {
                println!("task list is empty");
                return;
            }
            for t in &tasks {
                let status = if t.done { "[x]" } else { "[ ]" };
                println!("{} #{} {}", status, t.id, t.title);
            }
        }
        "done" => {
            let id: u32 = match args.get(1).and_then(|s| s.parse().ok()) {
                Some(id) => id,
                None => {
                    eprintln!("specify task ID");
                    return;
                }
            };
            let mut tasks = load_tasks();
            match tasks.iter_mut().find(|t| t.id == id) {
                Some(t) => {
                    t.done = true;
                    save_tasks(&tasks);
                    println!("task #{} completed", id);
                }
                None => eprintln!("task #{} not found", id),
            }
        }
        "remove" => {
            let id: u32 = match args.get(1).and_then(|s| s.parse().ok()) {
                Some(id) => id,
                None => {
                    eprintln!("specify task ID");
                    return;
                }
            };
            let mut tasks = load_tasks();
            let before = tasks.len();
            tasks.retain(|t| t.id != id);
            if tasks.len() < before {
                save_tasks(&tasks);
                println!("task #{} removed", id);
            } else {
                eprintln!("task #{} not found", id);
            }
        }
        _ => {
            eprintln!("unknown command: {}", args[0]);
            print_usage();
        }
    }
}
