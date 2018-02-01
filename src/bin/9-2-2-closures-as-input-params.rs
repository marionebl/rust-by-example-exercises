fn main() {
    use std::mem;

    let greeting = "hello";

    let mut farewell = String::from("goodbye");

    let diary = || {
        // Fn
        println!("I said {}.", greeting);

        // FnMut
        // farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzz");

        // FnOnce
        // mem::drop(farewell);
    };

    apply(diary);
}

fn apply<F>(mut f: F) where F: FnOnce() {
    f();
} 