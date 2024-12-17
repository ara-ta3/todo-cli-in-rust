use crate::domain::task::TaskList;
use anyhow::Result;

pub trait TaskRepository {
    fn fetch_all(&self) -> Result<&TaskList>;
}
