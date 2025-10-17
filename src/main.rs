use task_tracker_lib::TaskTracker;

fn main() {
    let mut task_tracker = match TaskTracker::new() {
        Ok(task_tracker) => task_tracker,
        Err(e) => {
            eprintln!("Error: {e}");
            return;
        }
    };

    task_tracker.start();
}
