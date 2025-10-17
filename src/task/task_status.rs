use std::{ fmt::Display, str::FromStr };

#[derive(PartialEq)]
pub enum TaskStatus {
    ToDo,
    InProgress,
    Done,
}

pub struct TaskStatusParseError;

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::ToDo => write!(f, "todo"),
            TaskStatus::InProgress => write!(f, "in-progess"),
            TaskStatus::Done => write!(f, "done"),
        }
    }
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::ToDo
    }
}

impl FromStr for TaskStatus {
    type Err = TaskStatusParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "todo" => Ok(TaskStatus::ToDo),
            "in-progress" => Ok(TaskStatus::InProgress),
            "done" => Ok(TaskStatus::Done),
            _ => Ok(TaskStatus::ToDo),
        }
    }
}
