mod box_mod {
    use std::fmt;

    pub struct Open<T> {
        pub contents: T,
    }

    pub struct Closed<T> {
        contents: T,
    }

    impl<T: fmt::Debug> Closed<T> {
        pub fn new(contents: T) -> Closed<T> {
            Closed { contents }
        }
        pub fn peek(&self) -> &T {
            &self.contents
        }
    }
}

fn main() {
    let open_box = box_mod::Open { contents: "public information" };
    println!("The open box contains: {}", open_box.contents);

    // error[E0451]: field `contents` of struct `box_mod::Closed` is private
    // let closed_box = box_mod::Closed { contents: "classified information" };

    let _closed_box = box_mod::Closed::new("classified information");

    // error[E0616]: field `contents` of struct `box_mod::Closed` is private
    // println!("The closed box contains: {}", _closed_box.contents);
    println!("Peek into closed box: {}", _closed_box.peek());
}