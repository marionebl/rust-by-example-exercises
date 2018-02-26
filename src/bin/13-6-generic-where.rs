use std::fmt::Debug;

trait Print {
    fn print(self);
}

impl<T> Print for T where Option<T>: Debug {
    fn print(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];
    vec.print();
}