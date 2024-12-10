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

#[test]
fn test_add() {
    let t1 = Todo {
        title: String::from("Hoge"),
    };
    let t2 = Todo {
        title: String::from("Fuga"),
    };
    let mut l = TodoList {
        list: vec![t1.clone()],
    };
    l.add(t2.clone());
    assert_eq!(l, TodoList { list: vec![t1, t2] });
}
