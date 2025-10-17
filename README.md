# Task Tracker CLI

A simple command-line tool for managing your tasks — add, update, delete, mark status, and list tasks. All data is stored in a JSON file in the current directory.

---

## 📦 Features / Requirements

The application supports:

- Add, update, and delete tasks  
- Mark a task as *in progress* or *done*  
- List all tasks  
- List tasks filtered by status: `todo`, `in-progress`, `done`  
- Tasks are persisted in a JSON file in the current working directory  
- If the JSON file doesn't exist yet, it is created automatically  
- Graceful error handling (invalid IDs, missing arguments, I/O problems)  

Each task has these properties:

| Field        | Description |
|--------------|--------------|
| `id`         | Unique identifier for the task |
| `description`| Short text describing the task |
| `status`     | Task status: one of `todo`, `in-progress`, `done` |
| `createdAt`  | Timestamp when the task was first created |
| `updatedAt`  | Timestamp when the task was last modified |

---

## 🛠️ Getting Started

### Prerequisites

- Rust (for building and running)  
- A terminal / command line environment  

### Build & Run

Clone the repo and build:

```bash
git clone https://github.com/andreas-wien/task_tracker.git
cd task_tracker
cargo build --release
```

```bash
# Adding a new task
task_tracker add "Buy groceries"

# Updating and deleting tasks
task_tracker update 1 "Buy groceries and cook dinner"
task_tracker delete 1

# Marking a task as in progress or done
task_tracker mark-in-progress 1
task_tracker mark-done 1

# Listing all tasks
task_tracker list

# Listing tasks by status
task_tracker list done
task_tracker list todo
task_tracker list in-progress
```

[https://roadmap.sh/projects/task-tracker](https://roadmap.sh/projects/task-tracker)