#[derive(Debug, PartialEq, Clone)]
pub struct Task {
    #[allow(dead_code)]
    pub title: String,
}

#[derive(Debug, PartialEq)]
pub struct TaskList {
    pub list: Vec<Task>,
}

impl TaskList {
    pub fn add(&mut self, t: Task) {
        self.list.push(t);
    }
}

#[test]
fn test_add() {
    let t1 = Task {
        title: String::from("Hoge"),
    };
    let t2 = Task {
        title: String::from("Fuga"),
    };
    let mut l = TaskList {
        list: vec![t1.clone()],
    };
    l.add(t2.clone());
    assert_eq!(l, TaskList { list: vec![t1, t2] });
}
