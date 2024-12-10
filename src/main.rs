pub mod domain {
    pub mod todo;
}

use domain::todo;

fn main() {
    let t1 = todo::Task {
        title: String::from("Hoge"),
    };
    let t2 = todo::Task {
        title: String::from("Fuga"),
    };
    let mut l = todo::TaskList { list: vec![t1] };
    l.add(t2);
    println!("{:?}", l);
}
