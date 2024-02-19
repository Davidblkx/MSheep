use crate::error::MSheepError;
use crate::tasker::task::*;
use crate::tasker::TaskContext;
use crate::error::Result;

use super::config::OrganizeConfig;

pub struct TaskOrganize {
    config: Option<OrganizeConfig>,
}

impl Task for TaskOrganize {
    fn name(&self) -> &str {
        "organize"
    }
    
    fn run(&mut self, step: TaskStep, context: &mut TaskContext) -> Result<TaskResult> {
        match step {
            TaskStep::Initialize => self.initialize(context),
            TaskStep::RunForFile => self.organize(context),
            _ => self.r_task_skip(step),
        }
    }
}

impl TaskOrganize {
    pub fn new() -> TaskOrganize {
        TaskOrganize {
            config: None,
        }
    }

    pub fn initialize(&mut self, context: &TaskContext) -> Result<TaskResult> {
        let config = OrganizeConfig::read(&context.config.get("organize"));
        self.config = Some(config);
        self.r_task_success(TaskStep::Initialize)
    }

    pub fn organize(&mut self, context: &mut TaskContext) -> Result<TaskResult> {
        let config = self.config.as_ref()
            .ok_or(MSheepError::TaskError("Not initialized".to_string()))?;

        let file = match context.current.as_mut() {
            Some(file) => file,
            None => return self.r_task_skip(TaskStep::RunForFile),
        };

        let data = match &file.data {
            Some(data) => data,
            None => return self.r_task_fail_m(TaskStep::RunForFile, "Metadata not found")
        };

        let dest_path = match super::path_builder::build_path(config.path.as_str(), data, file.path.extension()) {
            Ok(p) => p,
            Err(e) => {
                let message = format!("Failed to build destination path: {}", e);
                log::error!("{}", message);
                return self.r_task_fail_m(TaskStep::RunForFile, &message)
            }
        };

        if file.path == dest_path {
            log::trace!("[SKIP] File is organized: {:?}", file.path);
            return self.r_task_skip(TaskStep::RunForFile);
        }

        let word = if config.r#move { "Moved" } else { "Copied" };

        if context.options.dry_run {
            log::info!("!DRY_RUN[ORGANIZE] {} {:?} to {:?}", word, file.path, dest_path);
            return self.r_task_success(TaskStep::RunForFile);
        }

        super::organize::organize_file(config, &file.path, &dest_path)?;
        log::info!("[ORGANIZE] {} {:?} to {:?}", word, file.path, dest_path);

        self.r_task_success(TaskStep::RunForFile)
    }

    crate::impl_task_results!();
}