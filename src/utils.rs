use serde_json;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::io::{self, Read};

use crate::task::Task; // Make sure to import the Task struct

pub fn load_tasks() -> Result<Vec<Task>, io::Error> {
    // Try to open the file in read mode
    let mut file = match File::open("tasks.json") {
        Ok(file) => file,
        Err(e) => {
            // If the file doesn't exist, create a new file and return an empty Vec
            if e.kind() == io::ErrorKind::NotFound {
                File::create("tasks.json")?;
                return Ok(Vec::new());
            } else {
                return Err(e); // Return the error if it's not a "NotFound" error
            }
        }
    };

    // Read the file content into a string
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    // Deserialize the JSON data or return an empty Vec if deserialization fails
    let tasks = serde_json::from_str(&data).unwrap_or_else(|_| Vec::new());

    Ok(tasks)
}

pub fn save_tasks(tasks: &Vec<Task>) {
    let data = serde_json::to_string(&tasks).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("tasks.json")
        .unwrap();
    file.write_all(data.as_bytes()).unwrap();
}

pub fn get_next_id() -> usize {
    let tasks = load_tasks();
    tasks.unwrap_or_else(|_| Vec::new()).len() + 1
}
