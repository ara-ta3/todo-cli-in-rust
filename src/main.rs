use mytodocli::application::task_service::TaskService;
use mytodocli::domain::task::{Task, TaskList};
use mytodocli::infrastructure::task_repository_impl::TaskRepositoryOnMemory;

fn main() {
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
    let list = service.fetch_all();
    println!("{:?}", list);
}
