use crate::tasker::{Task, TaskStep, TaskContext, TaskResult, TaskResultKind};
use crate::error::Result;

pub struct TaskReadMetadata {}

impl Task for TaskReadMetadata {
    fn name(&self) -> &str {
        "read_metadata"
    }
    
    fn run(&mut self, step: TaskStep, context: &mut TaskContext) -> Result<TaskResult> {
        match step {
            TaskStep::RunForFile => self.load_metadata(context),
            _ => Ok(TaskResult {
                task: self.name().to_string(),
                step,
                kind: TaskResultKind::Skip,
                message: None,
            }),
        }
    }
}

impl TaskReadMetadata {
    pub fn load_metadata(&self, context: &mut TaskContext) -> Result<TaskResult> {
        match &mut context.current {
            Some(file) => {
                match file.load_data() {
                    Ok(_) => Ok(self.success()),
                    Err(e) => Ok(self.fail(&format!("Failed to load metadata: {}", e)))
                }
            },
            None => Ok(self.fail("No file to load metadata for")),
        }
    }

    
    pub fn success(&self) -> TaskResult {
        TaskResult {
            task: self.name().to_string(),
            step: TaskStep::RunForFile,
            kind: TaskResultKind::Success,
            message: None,
        }
    }

    pub fn fail(&self, message: &str) -> TaskResult {
        TaskResult {
            task: self.name().to_string(),
            step: TaskStep::RunForFile,
            kind: TaskResultKind::Fail,
            message: Some(message.to_string()),
        }
    }
}
