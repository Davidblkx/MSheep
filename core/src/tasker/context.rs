use bakunin_config::Value;
use crate::MusicFile;

use super::options::TaskOptions;

pub struct TaskContext {
    pub options: TaskOptions,
    pub current: Option<MusicFile>,
    pub config: Value,
}
