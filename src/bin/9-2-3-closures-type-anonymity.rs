fn main() {
    let x = 7;

    let print = || println!("{}", x);

    // TODO: Explore this
    // let apply = |f: Fn()| { f(); };

    apply(print);
}

fn apply<F: Fn()>(f: F) {
    f();
}