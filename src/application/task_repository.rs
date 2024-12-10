use crate::domain::task::TaskList;

pub trait TaskRepository {
    fn fetch_all(&self) -> &TaskList;
}
