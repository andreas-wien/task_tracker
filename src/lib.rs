mod task;
mod date_time;
mod cli;

use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

use crate::task::{ Task, TaskStatus };
use crate::cli::CLI;

enum TaskTrackerMode {
    Add,
    Update,
    Delete,
    List,
    MarkInProgress,
    MarkDone,
}

pub struct TaskTrackerModeParseError(String);

impl Display for TaskTrackerModeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No such mode: {}", self.0)
    }
}

impl Default for TaskTrackerMode {
    fn default() -> Self {
        TaskTrackerMode::Add
    }
}

impl FromStr for TaskTrackerMode {
    type Err = TaskTrackerModeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "add" => Ok(TaskTrackerMode::Add),
            "update" => Ok(TaskTrackerMode::Update),
            "delete" => Ok(TaskTrackerMode::Delete),
            "list" => Ok(TaskTrackerMode::List),
            "mark-in-progess" => Ok(TaskTrackerMode::MarkInProgress),
            "mark-done" => Ok(TaskTrackerMode::MarkDone),
            _ => Err(TaskTrackerModeParseError(s)),
        }
    }
}

pub struct TaskTracker {
    cli: CLI,
    tasks: HashMap<u32, Task>,
    mode: TaskTrackerMode,
}

impl TaskTracker {
    pub fn new() -> Result<Self, TaskTrackerModeParseError> {
        let cli = CLI::new();
        let tasks = TaskTracker::load_tasks();
        let mode = cli.arguments()[1].parse()?;
        Ok(TaskTracker {
            cli,
            tasks,
            mode,
        })
    }

    pub fn cli(&self) -> &CLI {
        &self.cli
    }

    pub fn start(&mut self) {
        match self.mode {
            TaskTrackerMode::Add => {
                let description = self.cli().arguments()[2].clone();
                self.add_task(&description);
            }
            TaskTrackerMode::Update => {
                let id = self.cli().arguments()[2].parse().unwrap_or_default();
                let description = self.cli().arguments()[3].clone();
                self.update_task(id, &description);
            }
            TaskTrackerMode::Delete => {
                let id = self.cli().arguments()[2].parse().unwrap_or_default();
                self.delete_task(id);
            }
            TaskTrackerMode::List => {
                let task_status = self.cli().arguments()[2].parse().unwrap_or_default();
                self.list_tasks(Some(task_status));
            }
            TaskTrackerMode::MarkInProgress => {
                let id = self.cli().arguments()[2].parse().unwrap_or_default();
                self.mark_task_in_progress(id);
            }
            TaskTrackerMode::MarkDone => {
                let id = self.cli().arguments()[2].parse().unwrap_or_default();
                self.mark_task_done(id);
            }
        }
    }

    // TODO Add, Update, and Delete tasks
    fn add_task(&mut self, description: &str) {
        TaskTracker::save_tasks();
    }

    fn update_task(&mut self, id: u32, description: &str) {
        let task = self.tasks.get_mut(&id);
        match task {
            Some(task) => task.set_description(description),
            None => todo!("Error: No task with such an ID"),
        }
        TaskTracker::save_tasks();
    }

    fn delete_task(&mut self, id: u32) {
        self.tasks.remove(&id);
        TaskTracker::save_tasks();
    }

    fn mark_task_in_progress(&mut self, id: u32) {
        let task = self.tasks.get_mut(&id);
        match task {
            Some(task) => task.set_status(TaskStatus::InProgress),
            None => todo!("Error: No task with such an ID"),
        }
        TaskTracker::save_tasks();
    }

    fn mark_task_done(&mut self, id: u32) {
        let task = self.tasks.get_mut(&id);
        match task {
            Some(task) => task.set_status(TaskStatus::Done),
            None => todo!("Error: No task with such an ID"),
        }
        TaskTracker::save_tasks();
    }

    fn list_tasks(&self, task_status: Option<TaskStatus>) {
        match task_status {
            Some(task_status) => {
                for (id, task) in &self.tasks {
                    if *task.status() == task_status {
                        print!("ID: {}, ", id);
                        task.print();
                    }
                }
            }
            None => {
                for (id, task) in &self.tasks {
                    print!("ID: {}, ", id);
                    task.print();
                }
            }
        }
    }
    fn save_tasks() {
        todo!("Implement saving tasks to json file in the current folder");
    }

    fn load_tasks() -> HashMap<u32, Task> {
        todo!("Implement loading tasks from json file in the current folder");
    }
}
