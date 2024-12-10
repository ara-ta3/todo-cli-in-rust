#[derive(Debug, PartialEq, Clone)]
pub struct Todo {
    #[allow(dead_code)]
    pub title: String,
}

#[derive(Debug, PartialEq)]
pub struct TodoList {
    pub list: Vec<Todo>,
}

impl TodoList {
    pub fn add(&mut self, t: Todo) {
        self.list.push(t);
    }
}
