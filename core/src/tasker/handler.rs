use crate::error::{Result, MSheepError};
use super::tasks::TaskType;

pub struct TaskHandler {
    tasks: Vec<TaskType>,
}

impl TaskHandler {
    pub fn new() -> Self {
        TaskHandler {
            tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: TaskType) {
        log::debug!("Adding task: {}", task.to_str());
        self.tasks.push(task);
    }

    pub fn add_by_name(&mut self, name: &str) -> Result<()> {
        match TaskType::from_str(name) {
            Some(task) => {
                self.add_task(task);
                Ok(())
            },
            None => Err(MSheepError::TaskError(format!("Unknown task: {}", name))),
        }
    }

    pub fn get_tasks(&self) -> &Vec<TaskType> {
        &self.tasks
    }
}