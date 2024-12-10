pub mod domain {
    pub mod todo;
}

use domain::todo;

fn main() {
    let t1 = todo::Todo {
        title: String::from("Hoge"),
    };
    let t2 = todo::Todo {
        title: String::from("Fuga"),
    };
    let mut l = todo::TodoList { list: vec![t1] };
    l.add(t2);
    println!("{:?}", l);
}
