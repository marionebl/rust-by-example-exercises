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

    let double = |x| x * 2;

    apply_to_three(double);
}

fn apply<F>(mut f: F) where F: FnOnce() {
    f();
} 

fn apply_to_three<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
    f(3)
}