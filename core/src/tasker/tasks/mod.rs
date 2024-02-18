mod task_read_metadata;
mod task_organize;

use crate::tasker::Task;

use task_organize::TaskOrganize;

pub enum TaskType {
    ReadMetadata,
    Organize,
}

impl TaskType {
    pub fn from_str(name: &str) -> Option<TaskType> {
        match name {
            "read_metadata" => Some(TaskType::ReadMetadata),
            "organize" => Some(TaskType::Organize),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            TaskType::ReadMetadata => "read_metadata",
            TaskType::Organize => "organize",
        }
    }

    pub fn to_task(&self) -> Box<dyn Task> {
        match self {
            TaskType::ReadMetadata => Box::new(task_read_metadata::TaskReadMetadata {}),
            TaskType::Organize => Box::new(TaskOrganize::new()),
        }
    }
}