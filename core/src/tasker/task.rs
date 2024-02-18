use super::context::TaskContext;
use crate::error::Result;

pub enum TaskStep {
    Initialize,
    RunForFile,
    Cleanup,
}

pub enum TaskResultKind {
    Success,
    Fail,
    Skip,
}

pub struct TaskResult {
    pub task: String,
    pub step: TaskStep,
    pub kind: TaskResultKind,
    pub message: Option<String>,
}

impl TaskResult {
    pub fn is_success(&self) -> bool {
        match self.kind {
            TaskResultKind::Success => true,
            TaskResultKind::Skip => true,
            _ => false,
        }
    }
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

#[macro_export]
macro_rules! impl_task_results {
    () => {
        #[allow(dead_code)]
        fn task_success(&self, step: TaskStep) -> TaskResult {
            TaskResult {
                task: self.name().to_string(),
                step,
                kind: TaskResultKind::Success,
                message: None,
            }
        }

        #[allow(dead_code)]
        fn task_fail(&self, step: TaskStep) -> TaskResult {
            TaskResult {
                task: self.name().to_string(),
                step,
                kind: TaskResultKind::Fail,
                message: None,
            }
        }

        #[allow(dead_code)]
        fn task_skip(&self, step: TaskStep) -> TaskResult {
            TaskResult {
                task: self.name().to_string(),
                step,
                kind: TaskResultKind::Skip,
                message: None,
            }
        }

        #[allow(dead_code)]
        fn task_success_m(&self, step: TaskStep, message: &str) -> TaskResult {
            TaskResult {
                task: self.name().to_string(),
                step,
                kind: TaskResultKind::Success,
                message: match message.is_empty() {
                    true => None,
                    false => Some(message.to_string()),
                },
            }
        }

        #[allow(dead_code)]
        fn task_fail_m(&self, step: TaskStep, message: &str) -> TaskResult {
            TaskResult {
                task: self.name().to_string(),
                step,
                kind: TaskResultKind::Fail,
                message: match message.is_empty() {
                    true => None,
                    false => Some(message.to_string()),
                },
            }
        }

        #[allow(dead_code)]
        fn task_skip_m(&self, step: TaskStep, message: &str) -> TaskResult {
            TaskResult {
                task: self.name().to_string(),
                step,
                kind: TaskResultKind::Skip,
                message: match message.is_empty() {
                    true => None,
                    false => Some(message.to_string()),
                },
            }
        }

        #[allow(dead_code)]
        fn r_task_success(&self, step: TaskStep) -> Result<TaskResult> {
            Ok(TaskResult {
                task: self.name().to_string(),
                step,
                kind: TaskResultKind::Success,
                message: None,
            })
        }

        #[allow(dead_code)]
        fn r_task_fail(&self, step: TaskStep) -> Result<TaskResult> {
            Ok(TaskResult {
                task: self.name().to_string(),
                step,
                kind: TaskResultKind::Fail,
                message: None,
            })
        }

        #[allow(dead_code)]
        fn r_task_skip(&self, step: TaskStep) -> Result<TaskResult> {
            Ok(TaskResult {
                task: self.name().to_string(),
                step,
                kind: TaskResultKind::Skip,
                message: None,
            })
        }

        #[allow(dead_code)]
        fn r_task_success_m(&self, step: TaskStep, message: &str) -> Result<TaskResult> {
            Ok(TaskResult {
                task: self.name().to_string(),
                step,
                kind: TaskResultKind::Success,
                message: match message.is_empty() {
                    true => None,
                    false => Some(message.to_string()),
                },
            })
        }

        #[allow(dead_code)]
        fn r_task_fail_m(&self, step: TaskStep, message: &str) -> Result<TaskResult> {
            Ok(TaskResult {
                task: self.name().to_string(),
                step,
                kind: TaskResultKind::Fail,
                message: match message.is_empty() {
                    true => None,
                    false => Some(message.to_string()),
                },
            })
        }

        #[allow(dead_code)]
        fn r_task_skip_m(&self, step: TaskStep, message: &str) -> Result<TaskResult> {
            Ok(TaskResult {
                task: self.name().to_string(),
                step,
                kind: TaskResultKind::Skip,
                message: match message.is_empty() {
                    true => None,
                    false => Some(message.to_string()),
                },
            })
        }
    };
}