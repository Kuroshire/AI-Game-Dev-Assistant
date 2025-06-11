use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub completed: bool,
    pub createdAt: String,
}

pub fn load_tasks_from_file() -> Result<Vec<Task>, std::io::Error> {
    println!("📥 load_tasks_from_file() was called");

    let path = "src/tasks/TaskSaveFile.json";
    match fs::read_to_string(path) {
        Ok(content) => {
            println!("📄 JSON content: {}", content);
            let tasks: Vec<Task> = serde_json::from_str(&content).unwrap_or_else(|e| {
                println!("❌ Failed to parse JSON: {}", e);
                vec![]
            });
            println!("✅ Loaded {} tasks", tasks.len());
            Ok(tasks)
        }
        Err(e) => {
            println!("❌ Failed to read file '{}': {}", path, e);
            Ok(vec![]) // Don't crash the app — return empty list
        }
    }
}

#[tauri::command]
pub fn get_tasks() -> Result<Vec<Task>, String> {
    match load_tasks_from_file() {
        Ok(tasks) => Ok(tasks),
        Err(err) => Err(format!("Failed to load tasks: {}", err)),
    }
}
