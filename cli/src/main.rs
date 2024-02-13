use std::path::PathBuf;

use msheep_core::tasker::{TaskHandler, TaskOptions, TaskType};

fn main() {
    let path = PathBuf::from("E:\\music");

    let mut handler = TaskHandler::new();
    handler.add_task(TaskType::Move);

    let options = TaskOptions {
        root: path,
        recursive: true,
        dry_run: false,
    };

    msheep_core::tasker::run_tasks(&handler, options).unwrap();
}
