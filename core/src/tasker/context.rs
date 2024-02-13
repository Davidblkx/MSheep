use crate::MusicFile;

use super::options::TaskOptions;

pub struct TaskContext {
    pub options: TaskOptions,
    pub current: Option<MusicFile>,
}
