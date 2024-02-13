use super::context::TaskContext;
use crate::error::Result;

pub enum TaskStep {
    Initialize,
    RunForFile,
    Cleanup,
}

pub struct TaskResult {
    pub task: String,
    pub step: TaskStep,
    pub success: bool,
    pub message: Option<String>,
}

/// A task is a unit of work that can be performed on a music file.
/// 
/// Lifecycle:
/// - invoke init for each task
/// - for each music file invoke run for each task (if init was successful)
/// - invoke end for each task
pub trait Task {
    fn name(&self) -> &str;
    fn run(&mut self, step: TaskStep, context: &mut TaskContext) -> Result<TaskResult>;
}