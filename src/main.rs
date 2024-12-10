pub mod domain {
    pub mod task;
}

pub mod application {
    pub mod task_repository;
    pub mod task_service;
}

pub mod infrastructure {
    pub mod task_repository_impl;
}

use application::task_service::TaskService;
use domain::task::{Task, TaskList};
use infrastructure::task_repository_impl::TaskRepositoryOnMemory;

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
