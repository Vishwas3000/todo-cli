use crate::utils::{get_next_id, load_tasks, save_tasks};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: usize,
    description: String,
    pub completed: bool,
    priority: Option<String>,
    due_date: Option<String>,
    progress: Option<usize>,
}

impl Task {
    pub fn new(id: usize, description: String) -> Task {
        Task {
            id,
            description,
            completed: false,
            priority: None,
            due_date: None,
            progress: None,
        }
    }
}

pub fn add_task(description: String) {
    let task = Task::new(get_next_id(), description);
    let tasks = load_tasks();
    match tasks {
        Ok(mut value) => {
            value.push(task);
            save_tasks(&value);
        }
        Err(e) => {
            eprintln!("Failed to load tasks: {}", e);
        }
    }
}

pub fn list_tasks() {
    let tasks = load_tasks();
    for task in tasks {
        println!("{:?}", task);
    }
}

pub fn complete_task(id: usize) {
    let tasks = load_tasks();
    match tasks {
        Ok(mut value) => {
            for task in value.iter_mut() {
                if task.id == id {
                    task.completed = true;
                }
            }
            save_tasks(&value);
        }
        Err(e) => {
            eprintln!("Failed to load tasks: {}", e);
        }
    }
}
