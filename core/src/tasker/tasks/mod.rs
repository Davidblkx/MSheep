mod task_move;

use crate::tasker::Task;

use task_move::TaskMove;

pub enum TaskType {
    Move,
}

impl TaskType {
    pub fn from_str(name: &str) -> Option<TaskType> {
        match name {
            "move" => Some(TaskType::Move),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            TaskType::Move => "move",
        }
    }

    pub fn to_task(&self) -> Box<dyn Task> {
        match self {
            TaskType::Move => Box::new(TaskMove {}),
        }
    }
}