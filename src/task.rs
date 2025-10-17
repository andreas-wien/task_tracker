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
    pub fn status(&self) -> &TaskStatus {
        &self.status
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
