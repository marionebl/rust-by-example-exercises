fn main() {
    fn increment (i: i32) -> i32 { i + 1 }

    let increment_annotated = |i: i32| -> i32 { i + 1 };
    let increment_inferred = |i| i + 1;

    let i = 1;

    println!("increment: {}", increment(i));
    println!("increment_annotated: {}", increment_annotated(i));
    println!("increment_inferred: {}", increment_inferred(i));

    let one = || 1;
    println!("Closure returning one: {}", one());

    use std::mem;

    let color = "green";

    let print = || println!("color: {}", color);

    print();
    print();

    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("count {}", count);
    };

    inc();
    inc();

    // error[E0499]: cannot borrow `count` as mutable more than once at a time
    // let reborrow = &mut count;

    let movable = Box::new(3);

    let consume = || {
        println!("movable: {:?}", movable);
        mem::drop(movable);
    };

    consume();
    // error[E0382]: use of moved value: `consume`
    // consume();
}