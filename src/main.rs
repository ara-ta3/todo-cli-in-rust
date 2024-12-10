pub mod domain {
    pub mod task;
}

use domain::task;

fn main() {
    let t1 = task::Task {
        title: String::from("Hoge"),
    };
    let t2 = task::Task {
        title: String::from("Fuga"),
    };
    let mut l = task::TaskList { list: vec![t1] };
    l.add(t2);
    println!("{:?}", l);
}
