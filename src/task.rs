mod task_status;

pub use task_status::TaskStatus;

use crate::date_time::DateTime;

pub struct Task {
    description: String,
    status: TaskStatus,
    created_at: DateTime,
    updated_at: DateTime,
}

impl Task {
    pub fn new(description: &str) -> Self {
        Task {
            description: description.to_owned(),
            status: TaskStatus::ToDo,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
    pub fn load(
        description: &str,
        status: &TaskStatus,
        created_at: &DateTime,
        updated_at: &DateTime
    ) -> Self {
        Task {
            description: description.to_owned(),
            status: status.clone(),
            created_at: created_at.clone(),
            updated_at: updated_at.clone(),
        }
    }

    pub fn status(&self) -> &TaskStatus {
        &self.status
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn created_at(&self) -> &DateTime {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime {
        &self.updated_at
    }

    pub fn set_status(&mut self, new_status: TaskStatus) {
        self.status = new_status;
    }

    pub fn set_description(&mut self, new_description: &str) {
        self.description = new_description.to_owned();
    }

    pub fn print(&self) {
        println!(
            "Description: {}, Status: {}, Created at: {}, Updated at: {}",
            self.description,
            self.status,
            self.created_at,
            self.updated_at
        );
    }
}
