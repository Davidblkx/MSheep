use bakunin_config::{value_map, Value};
use std::path::PathBuf;

use msheep_core::tasker::{TaskHandler, TaskOptions, TaskType};

fn main() {
    simple_logger::init_with_env().unwrap();

    let path = PathBuf::from("E:\\music\\1958 Miles (1958)");

    let mut handler = TaskHandler::new();
    handler.add_task(TaskType::ReadMetadata);
    handler.add_task(TaskType::Organize);

    let options = TaskOptions {
        root: path,
        recursive: true,
        dry_run: true,
    };

    let config = value_map! {
        organize: value_map!{
            path: "E:\\music\\%albumartist%\\%album% (%year%)\\%track% %title% - %artist% [%album%](%year%)",
            move: true,
        }
    };

    match msheep_core::tasker::run_tasks(&handler, options, config) {
        Ok(_) => log::info!("Tasks completed successfully"),
        Err(e) => log::error!("Error running tasks: {}", e),
    }
}
