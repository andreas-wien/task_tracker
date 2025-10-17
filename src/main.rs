use task_tracker_lib::{ TaskTracker, TaskTrackerCreationError };

fn main() {
    let mut task_tracker = match TaskTracker::new() {
        Ok(task_tracker) => task_tracker,
        Err(e) => {
            match e {
                TaskTrackerCreationError::TaskTrackerModeParseError(e) => eprintln!("{}", e),
                TaskTrackerCreationError::CLIInvalidArgumentsError(e) => eprintln!("{}", e),
            }
            return;
        }
    };

    match task_tracker.start() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {e}");
            return;
        }
    };
}
