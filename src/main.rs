use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

mod gui;
mod i18n;

use i18n::Localizer;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

fn data_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
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

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let loc = Localizer::new();

    if args.is_empty() {
        gui::run_gui();
        return;
    }

    match args[0].as_str() {
        "gui" => {
            gui::run_gui();
        }
        "add" => {
            let title = args[1..].join(" ");
            if title.is_empty() {
                eprintln!("{}", loc.translate("specify-task-text"));
                return;
            }
            let mut tasks = load_tasks();
            let task = Task {
                id: next_id(&tasks),
                title,
                done: false,
            };
            println!("{}", loc.translate_with_map("task-added", &std::collections::HashMap::from([
                ("id".to_string(), task.id.to_string()),
                ("title".to_string(), task.title.clone()),
            ])));
            tasks.push(task);
            save_tasks(&tasks);
        }
        "list" => {
            let tasks = load_tasks();
            if tasks.is_empty() {
                println!("{}", loc.translate("task-list-empty"));
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
                    eprintln!("{}", loc.translate("specify-task-id"));
                    return;
                }
            };
            let mut tasks = load_tasks();
            match tasks.iter_mut().find(|t| t.id == id) {
                Some(t) => {
                    t.done = true;
                    save_tasks(&tasks);
                    println!("{}", loc.translate_with_map("task-completed", &std::collections::HashMap::from([
                        ("id".to_string(), id.to_string()),
                    ])));
                }
                None => eprintln!("{}", loc.translate_with_map("task-not-found", &std::collections::HashMap::from([
                    ("id".to_string(), id.to_string()),
                ]))),
            }
        }
        "remove" => {
            let id: u32 = match args.get(1).and_then(|s| s.parse().ok()) {
                Some(id) => id,
                None => {
                    eprintln!("{}", loc.translate("specify-task-id"));
                    return;
                }
            };
            let mut tasks = load_tasks();
            let before = tasks.len();
            tasks.retain(|t| t.id != id);
            if tasks.len() < before {
                save_tasks(&tasks);
                println!("{}", loc.translate_with_map("task-removed", &std::collections::HashMap::from([
                    ("id".to_string(), id.to_string()),
                ])));
            } else {
                eprintln!("{}", loc.translate_with_map("task-not-found", &std::collections::HashMap::from([
                    ("id".to_string(), id.to_string()),
                ])));
            }
        }
        _ => {
            eprintln!("{}", loc.translate_with_map("unknown-command", &std::collections::HashMap::from([
                ("command".to_string(), args[0].clone()),
            ])));
            println!("{}", loc.translate("usage"));
            println!();
            println!("{}", loc.translate("commands"));
            println!("{}", loc.translate("cmd-add"));
            println!("{}", loc.translate("cmd-list"));
            println!("{}", loc.translate("cmd-done"));
            println!("{}", loc.translate("cmd-remove"));
            println!("{}", loc.translate("cmd-gui"));
        }
    }
}
