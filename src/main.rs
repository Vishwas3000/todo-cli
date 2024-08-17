use task::{add_task, complete_task, list_tasks};
use utils::get_next_id;

pub mod task;
mod utils;
fn main() {
    add_task(String::from("Buy milk"));
    get_next_id();
    complete_task(1);
    list_tasks();
}
