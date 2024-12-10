pub mod presentation {
    pub mod cli;
}

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
