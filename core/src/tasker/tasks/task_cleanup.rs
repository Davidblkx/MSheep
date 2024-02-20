use std::path::PathBuf;

use bakunin_config::Value;

use crate::tasker::{Task, TaskStep, TaskContext, TaskResult, TaskResultKind};
use crate::error::{MSheepError, Result};

pub struct CleanupConfig {
    pub empty_dirs: bool,
}

impl Default for CleanupConfig {
    fn default() -> Self {
        CleanupConfig {
            empty_dirs: true,
        }
    }
}

impl CleanupConfig {
    pub fn read(value: &Value) -> Self {
        let def = CleanupConfig::default();
        if !value.is_map() {
            log::warn!("Invalid cleanup config value, using default");
            return def;
        }

        CleanupConfig {
            empty_dirs: value.get("empty_dirs").try_into_bool().unwrap_or(def.empty_dirs),
        }
    }
}

pub struct TaskCleanup {
    config: Option<CleanupConfig>
}

impl Task for TaskCleanup {
    fn name(&self) -> &str {
        "cleanup"
    }
    
    fn run(&mut self, step: TaskStep, context: &mut TaskContext) -> Result<TaskResult> {
        match step {
            TaskStep::Initialize => self.initialize(context),
            TaskStep::Cleanup => self.cleanup(context),
            _ => self.r_task_skip(step)
        }
    }
}

impl TaskCleanup {
    pub fn new() -> TaskCleanup {
        TaskCleanup {
            config: None,
        }
    }

    fn initialize(&mut self, context: &mut TaskContext) -> Result<TaskResult> {
        let config = &context.config.get("cleanup");
        self.config = Some(CleanupConfig::read(config));
        self.r_task_success(TaskStep::Initialize)
    }

    fn cleanup(&self, context: &mut TaskContext) -> Result<TaskResult> {
        let config = self.config.as_ref()
            .ok_or(MSheepError::TaskError("Not initialized".to_string()))?;

        if !config.empty_dirs {
            return self.r_task_skip(TaskStep::Cleanup);
        }

        let root = &context.options.root;
        let is_dry_run = context.options.dry_run;
        self.remove_dir(root, is_dry_run)?;

        self.r_task_success(TaskStep::Cleanup)
    }

    fn remove_dir(&self, path: &PathBuf, dry_run: bool) -> Result<bool> {
        let mut is_empty = true;
        for entry in path.read_dir()? {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                is_empty = false;
                break;
            }
            
            if entry.file_type()?.is_dir() {
                let is_child_empty = self.remove_dir(&entry.path(), dry_run)?;
                if is_child_empty {
                    if !dry_run {
                        log::info!("[cleanup] Removing empty directory: {:?}", entry.path());
                        std::fs::remove_dir(&entry.path())?;
                    } else {
                        log::info!("!DRY_RUN[cleanup] Removing empty directory: {:?}", entry.path());
                    }
                } else {
                    is_empty = false;
                }
            }
        }
        Ok(is_empty)
    }

    crate::impl_task_results!();
}