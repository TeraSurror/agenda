use chrono::NaiveDate;

use crate::model::{Priority, Task};

#[derive(Debug)]
pub struct TaskStore {
    pub tasks: Vec<Task>,
}

impl TaskStore {
    pub fn new() -> Self {
        TaskStore { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, title: String, priority: Priority, due: Option<String>) {
        let due_date = due
            .map(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d"))
            .transpose()
            .unwrap();

        self.tasks
            .push(Task::new(1, title.clone(), priority, due_date))
    }
}
