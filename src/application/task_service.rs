use crate::application::task_repository::TaskRepository;
use crate::domain::task::TaskList;

pub struct TaskService<R: TaskRepository> {
    task_repository: R,
}

impl<R: TaskRepository> TaskService<R> {
    pub fn new(task_repository: R) -> Self {
        Self { task_repository }
    }

    pub fn fetch_all(&self) -> &TaskList {
        &self.task_repository.fetch_all()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::task::Task;
    use crate::infrastructure::task_repository_impl::TaskRepositoryOnMemory;

    #[test]
    fn test_fetch_all() {
        let repository = TaskRepositoryOnMemory::new(TaskList {
            list: vec![
                Task {
                    title: String::from("Hoge"),
                },
                Task {
                    title: String::from("Fuga"),
                },
            ],
        });
        let service = TaskService::new(repository);
        let actual = service.fetch_all();
        let expected = TaskList {
            list: vec![
                Task {
                    title: String::from("Hoge"),
                },
                Task {
                    title: String::from("Fuga"),
                },
            ],
        };
        assert_eq!(*actual, expected);
    }
}
