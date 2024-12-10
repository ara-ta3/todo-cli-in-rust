use crate::application::task_repository::TaskRepository;
use crate::domain::task::TaskList;

pub struct TaskRepositoryOnMemory {
    tasks: TaskList,
}

impl TaskRepositoryOnMemory {
    pub fn new(l: TaskList) -> Self {
        Self { tasks: l }
    }
}

impl TaskRepository for TaskRepositoryOnMemory {
    fn fetch_all(&self) -> &TaskList {
        &self.tasks
    }
}
