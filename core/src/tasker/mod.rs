pub mod context;
pub mod options;

pub mod task;
pub mod handler;
pub mod tasks;

pub use self::task::{Task, TaskStep, TaskResult, TaskResultKind};
pub use self::handler::TaskHandler;
pub use self::context::TaskContext;
pub use self::options::TaskOptions;
pub use self::tasks::TaskType;

use super::error::Result;

/// Run the tasks on the given files.
/// 
/// # Arguments
/// 
/// * `handler` - The task handler to use.
/// * `options` - The options for the tasks.
pub fn run_tasks(handler: &TaskHandler, options: TaskOptions, config: bakunin_config::Value) -> Result<()> {
    log::debug!("Preparing to run tasks");
    let file_list = options.build_finder().list()?;
    
    let mut context = TaskContext {
        current: None,
        options,
        config,
    };

    let mut tasks: Vec<Box<dyn Task>> = Vec::new();
    
    log::debug!("Initializing tasks");
    for t in handler.get_tasks() {
        log::trace!("Initializing task of type: {}", t.to_str());
        let mut task = t.to_task();
        let res = task.run(TaskStep::Initialize, &mut context)?;
        if !res.is_success() {
            log::warn!("[{}] Task failed to initialize: {}", res.task, res.message.unwrap_or_default());
        } else {
            tasks.push(task);
        }
    }

    if tasks.is_empty() {
        log::warn!("No tasks were initialized, exiting");
        return Ok(());
    }

    log::debug!("Running tasks for music files");
    for file in file_list {
        context.current = Some(file?);
        for task in &mut tasks {
            log::trace!("Running task: {}", task.name());
            let res = task.run(TaskStep::RunForFile, &mut context)?;
            if !res.is_success() {
                log::warn!("[{}] Task failed to run: {}", res.task, res.message.unwrap_or("Unknown error".to_string()));
            }
        }
    }

    log::debug!("Cleaning up tasks");
    for task in &mut tasks {
        log::trace!("Cleaning up task: {}", task.name());
        let res = task.run(TaskStep::Cleanup, &mut context)?;
        if !res.is_success() {
            log::warn!("[{}] Task failed to cleanup: {}", res.task, res.message.unwrap_or("Unknown error".to_string()));
        }
    }

    Ok(())
}