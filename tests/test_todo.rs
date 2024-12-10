use mytodocli::domain::todo::{Todo, TodoList};

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
