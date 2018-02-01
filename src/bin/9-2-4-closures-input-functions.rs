fn main() {
    let closure = || println!("I am a closure");
    apply(&closure);
    apply(&function);
    apply(&closure);
    apply(&function);
}

fn function() {
    println!("I am a function");
}

fn apply<F: Fn()>(f: &F) {
    f();
}