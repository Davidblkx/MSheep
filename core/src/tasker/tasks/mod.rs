mod task_read_metadata;
mod task_organize;
mod task_cleanup;

use crate::tasker::Task;

use task_read_metadata::TaskReadMetadata;
use task_organize::TaskOrganize;
use task_cleanup::TaskCleanup;

pub enum TaskType {
    ReadMetadata,
    Organize,
    Cleanup,
}

impl TaskType {
    pub fn from_str(name: &str) -> Option<TaskType> {
        match name {
            "read_metadata" => Some(TaskType::ReadMetadata),
            "organize" => Some(TaskType::Organize),
            "cleanup" => Some(TaskType::Cleanup),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            TaskType::ReadMetadata => "read_metadata",
            TaskType::Organize => "organize",
            TaskType::Cleanup => "cleanup",
        }
    }

    pub fn to_task(&self) -> Box<dyn Task> {
        match self {
            TaskType::ReadMetadata => Box::new(TaskReadMetadata {}),
            TaskType::Organize => Box::new(TaskOrganize::new()),
            TaskType::Cleanup => Box::new(TaskCleanup::new()),
        }
    }
}