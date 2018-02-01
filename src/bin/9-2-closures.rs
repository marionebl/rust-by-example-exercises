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
}