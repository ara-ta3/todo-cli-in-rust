use crate::application::task_service::TaskService;
use crate::domain::task::{Task, TaskList};
use crate::infrastructure::task_repository_impl::TaskRepositoryOnMemory;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    command: String,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
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

    let args = Args::parse();
    println!("{:?}", args);

    let list = service.fetch_all();
    println!("{:?}", list);
    Ok(())
}
