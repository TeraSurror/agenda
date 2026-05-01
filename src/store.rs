use std::{fs, path::PathBuf};

use chrono::NaiveDate;

use crate::{
    error::TaskError,
    model::{Priority, Status, Task},
};

pub fn add_task(title: String, priority: Priority, due: Option<String>) -> anyhow::Result<()> {
    let mut tasks = load_tasks()?;
    let due_date = due
        .map(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d"))
        .transpose()
        .map_err(|_| TaskError::InvalidDate)?;

    let id = next_id(&tasks);
    tasks.push(Task::new(id, title, priority, due_date));
    save_tasks(&tasks)?;

    println!("Added task with id {id} successfully.");

    Ok(())
}

pub fn list_tasks(filter: Option<String>) -> anyhow::Result<()> {
    let tasks = load_tasks()?;
    let filtered_tasks: Vec<&Task> = tasks
        .iter()
        .filter(|t| match &filter {
            Some(f) if f == "pending" => t.status == Status::Pending,
            Some(f) if f == "done" => t.status == Status::Done,
            _ => true,
        })
        .collect();

    if filtered_tasks.is_empty() {
        println!("No tasks found.");
        return Ok(());
    }

    filtered_tasks.iter().for_each(|task| {
        let status = match task.status {
            Status::Done => "[✓]",
            _ => "[ ]",
        };
        println!("{}: {} {}", task.id, task.title, status);
    });

    Ok(())
}

pub fn complete_task(id: u32) -> anyhow::Result<()> {
    let mut tasks = load_tasks()?;
    let task = tasks
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or(TaskError::NotFound(id))?;

    task.status = Status::Done;
    save_tasks(&tasks)?;
    println!("Complted task: {id}");

    Ok(())
}

pub fn delete_task(id: u32) -> anyhow::Result<()> {
    let mut tasks = load_tasks()?;
    let index = tasks
        .iter()
        .position(|t| t.id == id)
        .ok_or(TaskError::NotFound(id))?;

    let removed = tasks.remove(index);
    save_tasks(&tasks)?;

    println!("Deleted task {id}: {}", removed.title);

    Ok(())
}

fn data_path() -> Result<PathBuf, TaskError> {
    let dir = dirs::data_dir()
        .ok_or(TaskError::NoDataDir)?
        .join("task-cli");

    fs::create_dir_all(&dir).map_err(TaskError::Io)?;

    Ok(dir.join("tasks.json"))
}

fn load_tasks() -> Result<Vec<Task>, TaskError> {
    let path = data_path()?;

    if !path.exists() {
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(&path).map_err(TaskError::Io)?;
    let tasks = serde_json::from_str(&contents).map_err(TaskError::Json)?;

    Ok(tasks)
}

fn save_tasks(tasks: &[Task]) -> Result<(), TaskError> {
    let path = data_path()?;
    let json = serde_json::to_string_pretty(tasks).map_err(TaskError::Json)?;

    fs::write(path, json).map_err(TaskError::Io)?;

    Ok(())
}

fn next_id(tasks: &[Task]) -> u32 {
    tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
}
