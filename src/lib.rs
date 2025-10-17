mod task;
mod date_time;
mod cli;

use std::collections::HashMap;
use std::fmt::Display;
use std::fs::{ self, File };
use std::io::{ BufRead, BufReader };
use std::path::Path;
use std::str::FromStr;

use crate::date_time::DateTime;
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
    next_id: u32,
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
            next_id: 0,
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
                println!("{}", self.cli().arguments().len());
                let task_status = if self.cli().arguments().len() < 3 {
                    None
                } else {
                    Some(self.cli().arguments()[2].parse().unwrap_or_default())
                };
                self.list_tasks(task_status);
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

    fn add_task(&mut self, description: &str) {
        self.tasks.insert(self.next_id, Task::new(description));
        self.save_tasks();
    }

    fn update_task(&mut self, id: u32, description: &str) {
        let task = self.tasks.get_mut(&id);
        match task {
            Some(task) => task.set_description(description),
            None => todo!("Error: No task with such an ID"),
        }
        self.save_tasks();
    }

    fn delete_task(&mut self, id: u32) {
        self.tasks.remove(&id);
        self.save_tasks();
    }

    fn mark_task_in_progress(&mut self, id: u32) {
        let task = self.tasks.get_mut(&id);
        match task {
            Some(task) => task.set_status(TaskStatus::InProgress),
            None => todo!("Error: No task with such an ID"),
        }
        self.save_tasks();
    }

    fn mark_task_done(&mut self, id: u32) {
        let task = self.tasks.get_mut(&id);
        match task {
            Some(task) => task.set_status(TaskStatus::Done),
            None => todo!("Error: No task with such an ID"),
        }
        self.save_tasks();
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
    fn save_tasks(&self) {
        let mut json = String::new();
        json.push_str(r#"["#);

        let mut c = 1;
        for (id, task) in &self.tasks {
            let task_json = format!(
                r#"
    {{
        "id": "{}",
        "description": "{}",
        "status": "{}",
        "createdAt": "{}",
        "updatedAt": "{}"
    }}"#,
                id,
                task.description(),
                task.status(),
                task.created_at().to_string(),
                task.updated_at().to_string()
            );
            json.push_str(&task_json);
            if c != self.tasks.len() {
                json.push_str(r#",
                "#);
            }

            c += 1;
        }
        json.push_str(r#"
]"#);
        fs::write(Path::new("./tasks.json"), json).unwrap();
    }

    fn load_tasks() -> HashMap<u32, Task> {
        let mut tasks = HashMap::new();
        let file = File::open("./tasks.json");
        let file = match file {
            Ok(file) => file,
            Err(_) => {
                return tasks;
            }
        };

        let reader = BufReader::new(file);
        let mut id = 0;
        let mut description = String::new();
        let mut status = TaskStatus::ToDo;
        let mut created_at = DateTime::now();
        let mut updated_at = DateTime::now();
        let mut object_open = false;
        for line in reader.lines() {
            let line = line.unwrap_or_default();
            let line = line.trim();
            if line.contains("{") {
                object_open = true;
            }
            if line.contains("}") {
                tasks.insert(id, Task::load(&description, &status, &created_at, &updated_at));
                object_open = false;
            }
            if object_open {
                let words: (&str, &str) = line.split_once(":").unwrap_or_default();
                match words.0 {
                    "\"id\"" => {
                        id = words.1
                            .trim()
                            .strip_prefix("\"")
                            .unwrap_or_default()
                            .strip_suffix("\",")
                            .unwrap_or_default()
                            .parse()
                            .unwrap_or_default();
                    }
                    "\"description\"" => {
                        description = words.1
                            .trim()
                            .strip_prefix("\"")
                            .unwrap_or_default()
                            .strip_suffix("\",")
                            .unwrap_or_default()
                            .to_owned();
                    }
                    "\"status\"" => {
                        status = words.1
                            .trim()
                            .strip_prefix("\"")
                            .unwrap_or_default()
                            .strip_suffix("\",")
                            .unwrap_or_default()
                            .parse()
                            .unwrap_or_default();
                    }
                    "\"createdAt\"" => {
                        created_at = words.1
                            .trim()
                            .strip_prefix("\"")
                            .unwrap_or_default()
                            .strip_suffix("\",")
                            .unwrap_or_default()
                            .parse()
                            .unwrap_or_default();
                    }
                    "\"updatedAt\"" => {
                        updated_at = words.1
                            .trim()
                            .strip_prefix("\"")
                            .unwrap_or_default()
                            .strip_suffix("\"")
                            .unwrap_or_default()
                            .parse()
                            .unwrap_or_default();
                    }
                    _ => (),
                }
            }
        }

        tasks
    }
}
