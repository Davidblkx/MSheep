use crate::tasker::{Task, TaskStep, TaskContext, TaskResult};
use crate::error::Result;

pub struct TaskMove {}

impl Task for TaskMove {
    fn name(&self) -> &str {
        "move"
    }
    
    fn run(&mut self, step: TaskStep, _: &mut TaskContext) -> Result<TaskResult> {
        Ok(TaskResult {
            task: self.name().to_string(),
            step,
            success: true,
            message: None,
        })
    }
}
