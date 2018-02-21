use std::fmt::Debug;

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }
struct Triangle { length: f64, height: f64 }

fn main() {
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle { length: 3.0, height: 4.0 };

    print_debug(&rectangle);

    // error[E0277]: the trait bound `Triangle: std::fmt::Debug` is not satisfied
    // print_debug(&_triangle);
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}